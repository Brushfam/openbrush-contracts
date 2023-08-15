import {ApiPromise} from '@polkadot/api'
import {getSelectorByName, getSigners, SS58ToHex} from '../helpers'
import ConstructorsGovernance from '../../../typechain-generated/constructors/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'

import ConstructorsReceiver from '../../../typechain-generated/constructors/mock_receiver'
import ContractReceiver from '../../../typechain-generated/contracts/mock_receiver'

import {VoteType} from '../../../typechain-generated/types-arguments/my_governor'

import ConstructorsVotes from '../../../typechain-generated/constructors/my_psp22_votes'
import ContractVotes from '../../../typechain-generated/contracts/my_psp22_votes'
import BN from 'bn.js'
import {expect} from 'chai'
import {ProposalState} from '../../../typechain-generated/types-returns/my_governor'
import {GovernorHelper} from './helper'

describe('Governor', function () {

  async function setup(
    totalSupply = 100000,
    votingDelay = 0,
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

    const contractFactoryReceiver = new ConstructorsReceiver(api, deployer)
    const contractAddressReceiver = (await contractFactoryReceiver.new()).address
    const contractReceiver = new ContractReceiver(contractAddressReceiver, deployer, api)

    const helper = new GovernorHelper(contractGovernance)

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
    
  it('deployment check', async function () {
    const {
      api
    } = await setup()
    await api.disconnect()
  })

  describe('vote with signature', function () {
    afterEach('no other votes are cast for proposalId', async function () {
    // 
    })

    it('votes with an EOA signature', async function () {
    // 
    })

    it('votes with a valid EIP-1271 signature', async function () {
    // 
    })

    afterEach('no other votes are cast', async function () {
    //  
    })
  })

  describe('should revert', function () {
    describe('on propose', function () {
      it('if proposal already exists', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.propose(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })
    })

    describe('on vote', function () {
      it('if proposal does not exist', async function () {
        const {
          api,
          deployer,
          helper
        } = await setup()

        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if voting has not started', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if vote was already casted', async function () {
        const {
          api,
          bob,
          deployer,
          contractVotes,
          helper
        } = await setup()

        helper.addProposal(
          contractVotes.address,
          getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          [bob.address, new BN(1000), ''],
          '<description>#proposer=' + SS58ToHex(api, deployer.address))
        await expect(helper.propose(deployer)).to.eventually.be.fulfilled
        await helper.waitForSnapshot()
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if voting is over', async function () {
        const {
          api,
          bob,
          deployer,
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
        await helper.waitForDeadline()

        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.rejected

        await api.disconnect()
      })
    })

    describe('on vote by signature', function () {
      beforeEach(async function () {
        //
      })

      it('if signature does not match signer', async function () {
        //
      })

      it('if vote nonce is incorrect', async function () {
        //
      })
    })

    describe('on execute', function () {
      it('if proposal does not exist', async function () {
        const {
          api,
          deployer,
          helper
        } = await setup()

        await expect(helper.execute(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if quorum is not reached', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await expect(helper.execute(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if score not reached', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.against)).to.eventually.be.fulfilled
        await expect(helper.execute(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if voting is not over', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await expect(helper.execute(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if receiver revert without reason', async function () {
        //TODO
      })

      it('if receiver revert with reason', async function () {
        //TODO
      })

      it('if proposal was already executed', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await helper.waitForDeadline()
        await expect(helper.execute(deployer)).to.eventually.be.fulfilled
        await expect(helper.execute(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })
    })
  })

  describe('state', function () {
    it('Unset', async function () {
      const {
        api,
        deployer,
        helper
      } = await setup()

      await expect(helper.state(deployer)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('Pending & Active', async function () {
      const {
        api,
        bob,
        deployer,
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
      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.pending)
      await helper.waitForSnapshot()
      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.pending)
      await helper.waitForSnapshot(1)
      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.active)

      await api.disconnect()
    })

    it('Defeated', async function () {
      const {
        api,
        bob,
        deployer,
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
      await helper.waitForDeadline()
      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.active)
      await helper.waitForDeadline(1)
      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.defeated)

      await api.disconnect()
    })

    it('Succeeded', async function () {
      const {
        api,
        bob,
        deployer,
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

      await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
      await helper.waitForDeadline()

      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.active)
      await helper.waitForDeadline(1)

      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.succeeded)

      await api.disconnect()
    })

    it('Executed', async function () {
      const {
        api,
        bob,
        deployer,
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
      await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
      await helper.waitForDeadline()
      await expect(helper.execute(deployer)).to.eventually.be.fulfilled
      await expect(helper.state(deployer)).to.eventually.be.equals(ProposalState.executed)

      await api.disconnect()
    })
  })

  describe('cancel', function () {
    describe('public', function () {
      it('before proposal', async function () {
        const {
          api,
          deployer,
          helper
        } = await setup()

        await expect(helper.cancel(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('after proposal', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.cancel(deployer)).to.eventually.be.fulfilled

        await api.disconnect()
      })

      it('after proposal - restricted to proposer', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.cancel(bob)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('after vote started', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.cancel(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('after vote', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await expect(helper.cancel(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('after deadline', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await helper.waitForDeadline()
        await expect(helper.cancel(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('after execution', async function () {
        const {
          api,
          bob,
          deployer,
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
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await helper.waitForDeadline()
        await expect(helper.execute(deployer)).to.eventually.be.fulfilled
        await expect(helper.cancel(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })
    })
  })

  describe('proposal length', function () {
    it('empty', async function () {
      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.propose([], '<description>')).to.eventually.be.rejected
      
      await api.disconnect()
    })
  })

  describe('frontrun protection using description suffix', function () {
    describe('without protection', function () {
      describe('without suffix', function () {
        it('proposer can propose', async function () {
          const {api, helper} = await setup()

          await expect(helper.propose()).to.eventually.be.fulfilled
          
          await api.disconnect()
        })

        it('someone else can propose', async function () {
          const {api, helper, bob} = await setup()
    
          await expect(helper.propose(bob)).to.eventually.be.fulfilled
            
          await api.disconnect()
        })
      })

      describe('with different suffix', function () {
        it('proposer can propose', async function () {
          const {api, helper, deployer, contractReceiver} = await setup()

          helper.addProposal(
            contractReceiver.address,
            getSelectorByName(contractReceiver.abi.messages, 'mock_function'),
            [],
            '<description>#wrong-suffix=' + SS58ToHex(api, deployer.address)
          )

          await expect(helper.propose(deployer)).to.eventually.be.fulfilled

          await api.disconnect()
        })

        it('someone else can propose', async function () {
          const {api, helper, contractReceiver, alice, deployer} = await setup()

          helper.addProposal(
            contractReceiver.address,
            getSelectorByName(contractReceiver.abi.messages, 'mock_function'),
            [],
            '<description>#wrong-suffix=' + SS58ToHex(api, deployer.address)
          )

          await expect(helper.propose(alice)).to.eventually.be.fulfilled

          await api.disconnect()
        })
      })

      describe('with proposer suffix but bad address part', function () {
        it('propose can propose', async function () {
          const {api, helper, deployer, contractReceiver} = await setup()

          helper.addProposal(
            contractReceiver.address,
            getSelectorByName(contractReceiver.abi.messages, 'mock_function'),
            [],
            '<description>#proposer=' + '0x3C44CdDdB6a900fa2b585dd299e03d12FA429XYZ'
          )

          await expect(helper.propose(deployer)).to.eventually.be.fulfilled

          await api.disconnect()
        })

        it('someone else can propose', async function () {
          const {api, helper, contractReceiver, alice} = await setup()

          helper.addProposal(
            contractReceiver.address,
            getSelectorByName(contractReceiver.abi.messages, 'mock_function'),
            [],
            '<description>#proposer=' + '0x3C44CdDdB6a900fa2b585dd299e03d12FA429XYZ'
          )

          await expect(helper.propose(alice)).to.eventually.be.fulfilled

          await api.disconnect()
        })
      })
    })

    describe('with protection via proposer suffix', function () {
      it('proposer can propose', async function () {
        const {api, helper, deployer, contractGovernance} = await setup()

        helper.addProposal(
          contractGovernance.address,
          getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
          [100],
          '<description>#proposer=' + SS58ToHex(api, deployer.address)
        )

        await expect(helper.propose(deployer)).to.eventually.be.fulfilled

        await api.disconnect()
      })

      it('someone else cannot propose', async function () {
        const {api, helper, deployer, contractGovernance, alice} = await setup()

        helper.addProposal(
          contractGovernance.address,
          getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
          [100],
          '<description>#proposer=' + SS58ToHex(api, deployer.address)
        )

        console.log(SS58ToHex(api, deployer.address))

        await expect(helper.propose(alice)).to.eventually.be.rejected

        await api.disconnect()
      })
    })
  })

  describe('onlyGovernance updates', function () {
    it('setVotingDelay is protected', async function () {
      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.setVotingDelay(100)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('setVotingPeriod is protected', async function () {
      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.setVotingPeriod(100)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('setProposalThreshold is protected', async function () {
      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.setProposalThreshold(100)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('can setVotingDelay through governance', async function () {
      const {api, contractGovernance, deployer, helper, alice} = await setup()
      
      helper.addProposal(
        contractGovernance.address,
        getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
        [100],
        '<description>'
      )
      
      await expect(helper.propose(deployer)).to.eventually.be.fulfilled
      await expect(helper.waitForSnapshot()).to.eventually.be.fulfilled
      await expect(helper.castVote(alice, VoteType.for)).to.eventually.be.fulfilled
      await expect(helper.waitForDeadline()).to.eventually.be.fulfilled
      
      expect((await contractGovernance.query.votingDelay()).value.unwrapRecursively()).to.be.eq(new BN(100))

      await api.disconnect()
    })

    it('can setVotingPeriod through governance', async function () {
      const {api, contractGovernance, deployer, helper, alice} = await setup()

      helper.addProposal(
        contractGovernance.address,
        getSelectorByName(contractGovernance.abi.messages, 'set_voting_period'),
        [100],
        '<description>'
      )

      await expect(helper.propose(deployer)).to.eventually.be.fulfilled
      await expect(helper.waitForSnapshot()).to.eventually.be.fulfilled
      await expect(helper.castVote(alice, VoteType.for)).to.eventually.be.fulfilled
      await expect(helper.waitForDeadline()).to.eventually.be.fulfilled

      expect((await contractGovernance.query.votingPeriod()).value.unwrapRecursively()).to.be.eq(new BN(100))

      await api.disconnect()
    })

    it('cannot setVotingPeriod to 0 through governance', async function () {
      const {api, contractGovernance, deployer, helper, alice} = await setup()

      helper.addProposal(
        contractGovernance.address,
        getSelectorByName(contractGovernance.abi.messages, 'set_voting_period'),
        [0],
        '<description>'
      )

      await expect(helper.propose(deployer)).to.eventually.be.fulfilled
      await expect(helper.waitForSnapshot()).to.eventually.be.fulfilled
      await expect(helper.castVote(alice, VoteType.for)).to.eventually.be.fulfilled
      await expect(helper.waitForDeadline()).to.eventually.be.fulfilled

      await expect(helper.execute(deployer)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('can setProposalThreshold to 0 through governance', async function () {
      const {api, contractGovernance, deployer, helper, alice} = await setup()

      helper.addProposal(
        contractGovernance.address,
        getSelectorByName(contractGovernance.abi.messages, 'set_proposal_threshold'),
        [0],
        '<description>'
      )

      await expect(helper.propose(deployer)).to.eventually.be.fulfilled
      await expect(helper.waitForSnapshot()).to.eventually.be.fulfilled
      await expect(helper.castVote(alice, VoteType.for)).to.eventually.be.fulfilled
      await expect(helper.waitForDeadline()).to.eventually.be.fulfilled

      expect((await contractGovernance.query.proposalThreshold()).value.unwrapRecursively()).to.be.eq(new BN(0))

      await api.disconnect()
    })
  })
})

  