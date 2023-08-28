import {ApiPromise} from '@polkadot/api'
import {expect, getSelectorByName, getSigners, SS58ToHex} from '../helpers'
import ConstructorsVotes from '../../../typechain-generated/constructors/my_psp22_votes'
import ContractVotes from '../../../typechain-generated/contracts/my_psp22_votes'
import ConstructorsGovernance from '../../../typechain-generated/constructors/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import ConstructorsReceiver from '../../../typechain-generated/constructors/mock_receiver'
import ContractReceiver from '../../../typechain-generated/contracts/mock_receiver'
import {GovernorHelper} from './helper'
import BN from 'bn.js'
import {VoteType} from '../../../typechain-generated/types-arguments/my_governor'
import {number} from '@noble/hashes/_assert'

describe('Counting', function () {
  async function setup(
    totalSupply = 30,
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

    helper.addProposal(
      contractAddressReceiver,
      getSelectorByName(contractReceiver.abi.messages, 'mock_function'),
      [],
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

  describe('performs counting operations',async function () {

    it('has_voted', async () => {
      const {
        api,
        alice,
        bob,
        deployer,
        contractGovernance,
        contractVotes,
        helper
      } = await setup()

      helper.addProposal(
        contractVotes.address,
        getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        [bob.address, new BN(1000), ''],
        '<description>#proposer=' + SS58ToHex(api, deployer.address)
      )
      await expect(helper.propose(deployer)).to.eventually.be.fulfilled
      await helper.waitForSnapshot()
      await expect(helper.castVote(alice, VoteType.for)).to.eventually.be.fulfilled

      await expect(await helper.hasVoted(alice)).to.equals(true)
      await expect(await helper.hasVoted(bob)).to.equals(false)

      await api.disconnect()
    })

    it('proposal_votes', async () => {
      const {
        api,
        alice,
        bob,
        deployer,
        contractGovernance,
        contractVotes,
        helper
      } = await setup()

      helper.addProposal(
        contractVotes.address,
        getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        [bob.address, new BN(1000), ''],
        '<description>#proposer=' + SS58ToHex(api, deployer.address)
      )
      await expect(helper.propose(deployer)).to.eventually.be.fulfilled
      await helper.waitForSnapshot(1)
      await helper.increaseBlockTimestamp(2)

      await expect(await helper.getVotes(alice)).to.equals(10)


      await expect(helper.castVote(alice, VoteType.for)).to.eventually.be.fulfilled

      await expect(await helper.hasVoted(alice)).to.equals(true)
      await expect(await helper.hasVoted(bob)).to.equals(false)

      await expect(await helper.proposalVotes()).to.equal(Array.from([10, 0, 0]).toString())

      await expect(helper.castVote(bob, VoteType.against)).to.eventually.be.fulfilled

      await expect(await helper.hasVoted(alice)).to.equals(true)
      await expect(await helper.hasVoted(bob)).to.equals(true)

      await expect(await helper.proposalVotes()).to.equal(Array.from([10, 10, 0]).toString())

      await expect(helper.castVote(deployer, VoteType.abstain)).to.eventually.be.fulfilled

      await expect(await helper.hasVoted(alice)).to.equals(true)
      await expect(await helper.hasVoted(bob)).to.equals(true)
      await expect(await helper.hasVoted(deployer)).to.equals(true)

      await expect(await helper.proposalVotes()).to.equal(Array.from([10, 10, 10]).toString())

      await api.disconnect()
    })
  })
})