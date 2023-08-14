import {ApiPromise} from '@polkadot/api'
import {getSelectorByName, getSigners, Uint8ArrayToString} from '../helpers'
import ConstructorsGovernance from '../../../typechain-generated/constructors/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import type * as ArgumentTypes from '../../../typechain-generated/types-arguments/my_governor'
import {Transaction, VoteType} from '../../../typechain-generated/types-arguments/my_governor'

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
    const helper = new GovernorHelper(contractGovernance)

    return {
      api,
      alice,
      bob,
      deployer,
      contractGovernance,
      contractAddressGovernance,
      contractVotes,
      contractAddressVotes,
      helper
    }
  }
  
  beforeEach(async function () {
  //   
  })
    
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
          contractGovernance,
          contractVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
        await expect(helper.propose(deployer)).to.eventually.be.fulfilled
        await expect(helper.propose(deployer)).to.eventually.be.rejected

        await api.disconnect()
      })
    })

    describe('on vote', function () {
      it('if proposal does not exist', async function () {
        const {
          api,
          bob,
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
        await expect(helper.propose(deployer)).to.eventually.be.fulfilled
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('if vote was already casted', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
        await expect(helper.propose(deployer)).to.eventually.be.fulfilled
        await helper.waitForSnapshot()
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.fulfilled
        await expect(helper.castVote(deployer, VoteType.for)).to.eventually.be.rejected

        await api.disconnect()
      });

      it('if voting is over', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
            helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
            helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
            helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
        bob,
        deployer,
        contractGovernance,
        contractAddressGovernance,
        contractVotes,
        contractAddressVotes
      } = await setup(100000, 0, 100, 0, 0)
      const transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000
      }]

      const blake2 = require('blake2')
      const descriptionHash = blake2.createHash('blake2b')
      descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

      const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

      await expect(contractGovernance.tx.state(proposalId)).to.eventually.be.rejected

      await api.disconnect()
    })

    it('Pending & Active', async function () {
      const {
        api,
        bob,
        deployer,
        contractGovernance,
        contractAddressGovernance,
        contractVotes,
        contractAddressVotes,
        helper
      } = await setup()

      const transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      const blake2 = require('blake2')
      const descriptionHash = blake2.createHash('blake2b')
      descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

      const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.pending)
      const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
      await contractGovernance.tx.setBlockTimestamp(proposalSnapshot)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.pending)
      await contractGovernance.tx.setBlockTimestamp(proposalSnapshot + 1)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.active)
      await api.disconnect()
    })

    it('Defeated', async function () {
      const {
        api,
        bob,
        deployer,
        contractGovernance,
        contractAddressGovernance,
        contractVotes,
        contractAddressVotes
      } = await setup(100000, 0, 100, 0, 0)

      const transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      const blake2 = require('blake2')
      const descriptionHash = blake2.createHash('blake2b')
      descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

      const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()
      const proposalDeadline = (await contractGovernance.query.proposalDeadline(proposalId)).value.unwrapRecursively().unwrapRecursively()
      await contractGovernance.tx.setBlockTimestamp(proposalDeadline)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.active)
      await contractGovernance.tx.setBlockTimestamp(proposalDeadline + 1)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.defeated)
      await api.disconnect()
    })

    it('Succeeded', async function () {
      const {
        api,
        bob,
        deployer,
        contractGovernance,
        contractAddressGovernance,
        contractVotes,
        contractAddressVotes
      } = await setup(100000, 0, 100, 0, 0)

      const transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      const blake2 = require('blake2')
      const descriptionHash = blake2.createHash('blake2b')
      descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

      const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()
      const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
      await contractGovernance.tx.setBlockTimestamp(proposalSnapshot)
      await contractGovernance.tx.castVote(proposalId, VoteType.for)
      const proposalDeadline = (await contractGovernance.query.proposalDeadline(proposalId)).value.unwrapRecursively().unwrapRecursively()
      await contractGovernance.tx.setBlockTimestamp(proposalDeadline)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.active)
      await contractGovernance.tx.setBlockTimestamp(proposalDeadline + 1)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.succeeded)
      await api.disconnect()
    })

    it('Executed', async function () {
      const {
        api,
        bob,
        deployer,
        contractGovernance,
        contractAddressGovernance,
        contractVotes,
        contractAddressVotes
      } = await setup(100000, 0, 100, 0, 0)

      const transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      const blake2 = require('blake2')
      const descriptionHash = blake2.createHash('blake2b')
      descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

      const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()
      const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
      await contractGovernance.tx.setBlockTimestamp(proposalSnapshot)
      await contractGovernance.tx.castVote(proposalId, VoteType.for)
      const proposalDeadline = (await contractGovernance.query.proposalDeadline(proposalId)).value.unwrapRecursively().unwrapRecursively()
      await contractGovernance.tx.setBlockTimestamp(proposalDeadline)
      await contractGovernance.tx.execute(transactions, descriptionHash)
      await expect((await contractGovernance.query.state(proposalId)).value.unwrapRecursively().unwrapRecursively()).to.have.equals(ProposalState.executed)
      await api.disconnect()
    })
  })

  describe('cancel', function () {
    describe('public', function () {
      it('before proposal', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
        await expect(helper.propose(deployer)).to.eventually.be.fulfilled
        await expect(helper.cancel(deployer)).to.eventually.be.fulfilled

        await api.disconnect()
      })

      it('after proposal - restricted to proposer', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
        await expect(helper.propose(deployer)).to.eventually.be.fulfilled
        await expect(helper.cancel(bob)).to.eventually.be.rejected

        await api.disconnect()
      })

      it('after vote started', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
            helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
          helper
        } = await setup()

        helper.addProposal(
            contractVotes.address,
            getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
            [bob.address, new BN(1000), ''],
            '<description>#propser=' + Uint8ArrayToString(deployer.addressRaw))
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
      const {api, helper, deployer} = await setup()

      helper.addProposal('', [], [], '<description>')

      await expect(helper.propose(deployer)).to.eventually.be.rejected
      
      await api.disconnect()
    })
  })

  describe('frontrun protection using description suffix', function () {
    describe('without protection', function () {
      describe('without suffix', function () {
        it('proposer can propose', async function () {
          const {api, helper, deployer} = await setup()

          await expect(helper.propose(deployer)).to.eventually.be.fulfilled
          
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
          const {api, helper, deployer, contractGovernance} = await setup()

          helper.addProposal(
            contractGovernance.address,
            getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
            [100],
            '<description>#wrong-suffix=' + Uint8ArrayToString(deployer.addressRaw)
          )

          await expect(helper.propose(deployer)).to.eventually.be.fulfilled

          await api.disconnect()
        })

        it('someone else can propose', async function () {
          const {api, helper, deployer, contractGovernance, alice} = await setup()

          helper.addProposal(
            contractGovernance.address,
            getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
            [100],
            '<description>#wrong-suffix=' + Uint8ArrayToString(deployer.addressRaw)
          )

          await expect(helper.propose(alice)).to.eventually.be.fulfilled

          await api.disconnect()
        })
      })

      describe('with proposer suffix but bad address part', function () {
        it('propose can propose', async function () {
          const {api, helper, deployer, contractGovernance} = await setup()

          helper.addProposal(
            contractGovernance.address,
            getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
            [100],
            '<description>#proposer=' + Uint8ArrayToString([0,1,2,3] as unknown as Uint8Array)
          )

          await expect(helper.propose(deployer)).to.eventually.be.fulfilled

          await api.disconnect()
        })

        it('someone else can propose', async function () {
          const {api, helper, deployer, contractGovernance, alice} = await setup()

          helper.addProposal(
            contractGovernance.address,
            getSelectorByName(contractGovernance.abi.messages, 'set_voting_delay'),
            [100],
            '<description>#proposer=' + Uint8ArrayToString([0,1,2,3] as unknown as Uint8Array)
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
          '<description>#proposer=' + Uint8ArrayToString(deployer.addressRaw)
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
          '<description>#proposer=' + Uint8ArrayToString(deployer.addressRaw)
        )

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

  