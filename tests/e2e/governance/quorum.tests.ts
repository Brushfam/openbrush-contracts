import ConstructorsGovernance from '../../../typechain-generated/constructors/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import {ApiPromise} from '@polkadot/api'
import {expect, getMessageByName, getSigners} from '../helpers'
import ConstructorsVotes from '../../../typechain-generated/constructors/my_psp22_votes'
import ContractVotes from '../../../typechain-generated/contracts/my_psp22_votes'
import ConstructorsReceiver from '../../../typechain-generated/constructors/mock_receiver'
import ContractReceiver from '../../../typechain-generated/contracts/mock_receiver'
import {GovernorHelper} from './helper'
import {VoteType} from '../../../typechain-generated/types-arguments/my_governor'


describe('QuorumImpl', () => {
  const  TOTAL_SUPPLY = 100000
  const  VOTING_DELAY = 10
  const  VOTING_PERIOD = 10
  const  PROPOSAL_THRESHOLD = 0
  const  NUMRATOR = 0

  async function setup(
    totalSupply = 100000,
    votingDelay = 10,
    votingPeriod = 10,
    proposalThreshold = 0,
    numrator = 0
  ){
    const api = await ApiPromise.create()

    const signers = getSigners()
    const deployer = signers[0]
    const alice = signers[1]
    const bob = signers[2]

    const contractFactoryVotes = new ConstructorsVotes(api, deployer)
    const contractAddressVotes = (await contractFactoryVotes.new(totalSupply)).address
    const contractVotes = new ContractVotes(contractAddressVotes, deployer, api)

    const contractFactoryGovernance = new ConstructorsGovernance(api, deployer)
    const contractAddressGovernance = (await contractFactoryGovernance.new(contractAddressVotes, votingDelay, votingPeriod, proposalThreshold, numrator)).address
    const contractGovernance = new ContractGovernance(contractAddressGovernance, deployer, api)

    await contractVotes.tx.setBlockTimestamp((await contractGovernance.query.blockTimestamp()).value.ok!)

    const contractFactoryReceiver = new ConstructorsReceiver(api, deployer)
    const contractAddressReceiver = (await contractFactoryReceiver.new()).address
    const contractReceiver = new ContractReceiver(contractAddressReceiver, deployer, api)

    const helper = new GovernorHelper(contractGovernance, contractVotes)

    await helper.delegate(contractVotes, deployer, alice, 10)
    await helper.delegate(contractVotes, deployer, bob, 10)
    await helper.delegate(contractVotes, deployer, deployer, 10)

    const callParams = helper.paramsToInput(getMessageByName(contractReceiver.abi.messages, 'mock_function').toU8a([]))

    helper.addProposal(
      contractAddressReceiver,
      callParams.selector,
      callParams.data,
      '<description>'
    )

    return {
      api,
      alice,
      bob,
      deployer,
      contractGovernance,
      contractAddressGovernance,
      contractVotes,
      contractAddressVotes,
      contractReceiver,
      helper
    }
  }

  it('deployment check', async function () {
    const {
      api,
      contractGovernance,
      alice,
      bob,
      deployer,
      contractVotes
    } = await setup()

    await expect((await contractGovernance.query.votingDelay()).value.ok!).to.equals(VOTING_DELAY)
    await expect((await contractGovernance.query.votingPeriod()).value.ok!).to.equals(VOTING_PERIOD)
    await expect((await contractGovernance.query.proposalThreshold()).value.ok!.rawNumber.toNumber()).to.equals(PROPOSAL_THRESHOLD)
    await expect((await contractGovernance.query.quorum(0)).value.ok!.ok!.rawNumber.toNumber()).to.equals(0)
    await expect((await contractGovernance.query.quorumNumerator()).value.ok!.rawNumber.toNumber()).to.equals(NUMRATOR)

    expect((await contractVotes.query.getVotes(alice.address)).value.ok!.toNumber()).to.be.eq(10)
    expect((await contractVotes.query.getVotes(bob.address)).value.ok!.toNumber()).to.be.eq(10)
    expect((await contractVotes.query.getVotes(deployer.address)).value.ok!.toNumber()).to.be.eq(99980)

    await api.disconnect()
  })

  it('quorum reached', async function () {
    const { api, deployer, helper} = await setup()

    await expect(helper.propose()).to.eventually.be.fulfilled

    await helper.waitForSnapshot()

    await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled

    await helper.waitForDeadline(1)

    await expect(helper.execute()).to.eventually.be.fulfilled

    await api.disconnect()
  })

  it('quorum not reached', async function () {
    const { api, deployer, helper} = await setup()

    await expect(helper.propose()).to.eventually.be.fulfilled

    await helper.waitForSnapshot()

    await expect(helper.castVote(deployer, VoteType.against)).to.eventually.be.fulfilled

    await helper.waitForDeadline(1)

    await expect(helper.execute()).to.eventually.be.rejected

    await api.disconnect()
  })

  describe('onlyGovernance updates', function () {
    it('updateQuorumNumerator is protected', async function () {

      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.updateQuorumNumerator(1)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('can updateQuorumNumerator through governance', async function () {

      const {api, deployer, contractGovernance, helper} = await setup()

      const callParams = helper.paramsToInput(getMessageByName(contractGovernance.abi.messages, 'update_quorum_numerator').toU8a([1]))

      helper.addProposal(contractGovernance.address, callParams.selector, callParams.data, '<description>')

      await expect(helper.propose()).to.eventually.be.fulfilled

      await helper.waitForSnapshot()

      await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled

      await helper.waitForDeadline(1)

      await expect(helper.execute()).to.eventually.be.fulfilled

      expect((await contractGovernance.query.quorumNumerator()).value.ok!.toNumber()).to.be.equal(1)
      expect((await contractGovernance.query.quorumDenominator()).value.ok!.toNumber()).to.be.equal(100)

      await api.disconnect()
    })

    it('cannot updateQuorumNumerator over the maximum', async function () {
      const {api, deployer, contractGovernance, helper} = await setup()

      const callParams = helper.paramsToInput(getMessageByName(contractGovernance.abi.messages, 'update_quorum_numerator').toU8a([101]))

      helper.addProposal(contractGovernance.address, callParams.selector, callParams.data, '<description>')

      await expect(helper.propose()).to.eventually.be.fulfilled

      await helper.waitForSnapshot()

      await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled

      await helper.waitForDeadline(1)

      await expect(helper.execute()).to.eventually.be.fulfilled

      expect((await contractGovernance.query.quorumNumerator()).value.ok!.toNumber()).to.be.eq(0)

      await api.disconnect()
    })
  })
})
