import {ApiPromise} from "@polkadot/api";
import {getSelectorByName, getSigners, Uint8ArrayToString} from "../helpers";
import ConstructorsGovernance from "../../../typechain-generated/constructors/my_governor";
import ContractGovernance from "../../../typechain-generated/contracts/my_governor";
import type * as ArgumentTypes from '../../../typechain-generated/types-arguments/my_governor';
import {Transaction, VoteType} from "../../../typechain-generated/types-arguments/my_governor";

import ConstructorsVotes from "../../../typechain-generated/constructors/my_psp22_votes";
import ContractVotes from "../../../typechain-generated/contracts/my_psp22_votes";
import BN from "bn.js";
import {expect} from "chai";
import {ProposalState} from "../../../typechain-generated/types-returns/my_governor";

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
    const bob = signers[1]
    const contractFactoryVotes = new ConstructorsVotes(api, deployer)
    const contractAddressVotes = (await contractFactoryVotes.new(totalSupply)).address
    const contractVotes = new ContractVotes(contractAddressVotes, deployer, api)
    const contractFactoryGovernance = new ConstructorsGovernance(api, deployer)
    const contractAddressGovernance = (await contractFactoryGovernance.new(contractAddressVotes, votingDelay, votingPeriod, proposalThreshold, numrator)).address
    const contractGovernance = new ContractGovernance(contractAddressGovernance, deployer, api)

    return {
      api,
      bob,
      deployer,
      contractGovernance,
      contractAddressGovernance,
      contractVotes,
      contractAddressVotes
    }
  }
  
  beforeEach(async function () {
  //   
  })
    
  it('deployment check', async function () {
    const {
      api,
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
              contractAddressGovernance,
              contractVotes,
              contractAddressVotes,
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
        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.rejected

       await api.disconnect()
      })
    })

    describe('on vote', function () {
      it('if proposal does not exist', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes
        } = await setup()
        const proposalId : number[] = []
        for(let i = 0; i < 32; i++) {
          proposalId.push(0)
        }
        console.log(proposalId)
        await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.rejected
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
          contractAddressVotes
        } = await setup(100000, 29384987, 10, 0, 0)
        const transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000
        }]
        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled
        const proposalId : number[] = []
        for(let i = 0; i < 32; i++) {
          proposalId.push(0)
        }
        await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.rejected
       await api.disconnect()
      })

      it('if voting is over', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
        } = await setup(100000, 0, 100000, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = await contractGovernance.query.hashProposal(transactions, descriptionHash)
        console.log(proposalId.value.unwrapRecursively().unwrapRecursively())

        await contractGovernance.tx.increaseBlockTimestamp(100001)

        await expect(contractGovernance.tx.castVote(proposalId.value.unwrapRecursively().unwrapRecursively(), VoteType.for)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        const proposalId : number[] = []
        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]
        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        await expect(contractGovernance.tx.execute(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]
        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.fulfilled
        
        await expect(contractGovernance.tx.execute(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]
        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot + 1)



        await expect(contractGovernance.tx.castVote(proposalId, VoteType.against)).to.eventually.be.fulfilled

        await expect(contractGovernance.tx.execute(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]
        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot + 1)

        await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.fulfilled

        await expect(contractGovernance.tx.execute(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]
        await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot + 1)

        await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.fulfilled
        const deadline = (await contractGovernance.query.proposalDeadline(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(deadline + 1)

        await expect(contractGovernance.tx.execute(transactions, descriptionHash)).to.eventually.be.rejected

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
        contractAddressVotes,
      } = await setup(100000, 0, 100, 0, 0)
      let transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000,
      }]

      var blake2 = require('blake2')
      var descriptionHash = blake2.createHash('blake2b')
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
      } = await setup(100000, 0, 100, 0, 0)

      let transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000,
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      var blake2 = require('blake2')
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
        contractAddressVotes,
      } = await setup(100000, 0, 100, 0, 0)

      let transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000,
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      var blake2 = require('blake2')
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
        contractAddressVotes,
      } = await setup(100000, 0, 100, 0, 0)

      let transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000,
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      var blake2 = require('blake2')
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
        contractAddressVotes,
      } = await setup(100000, 0, 100, 0, 0)

      let transactions: Array<ArgumentTypes.Transaction> = [{
        callee: contractVotes.address,
        selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
        destination: contractVotes.address,
        input: [bob.address, new BN(1000), ''], // [to, value, data]
        transferredValue: 0,
        gasLimit: 1000000000000,
      }]
      await expect(contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.fulfilled

      var blake2 = require('blake2')
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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        await expect(contractGovernance.tx.cancel(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))

        await expect(contractGovernance.tx.cancel(transactions, descriptionHash)).to.eventually.be.fulfilled

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))

        await expect(contractGovernance.withAddress(bob.address).tx.cancel(transactions, descriptionHash)).to.eventually.be.rejected

        await api.disconnect()
      });

      it('after vote started', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot + 1)

        await expect(contractGovernance.tx.cancel(transactions, descriptionHash)).to.eventually.be.rejected

        await api.disconnect()
      });

      it('after vote', async function () {
        const {
          api,
          bob,
          deployer,
          contractGovernance,
          contractAddressGovernance,
          contractVotes,
          contractAddressVotes,
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot)

        await contractGovernance.tx.castVote(proposalId, VoteType.for)

        await expect(contractGovernance.tx.cancel(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot)

        await contractGovernance.tx.castVote(proposalId, VoteType.for)

        const proposalDeadline = (await contractGovernance.query.proposalDeadline(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalDeadline)

        await expect(contractGovernance.tx.cancel(transactions, descriptionHash)).to.eventually.be.rejected

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
        } = await setup(100000, 0, 100, 0, 0)

        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]

        var blake2 = require('blake2')
        const descriptionHash = blake2.createHash('blake2b')
        descriptionHash.update(Buffer.from('#proposer=' + Uint8ArrayToString(deployer.addressRaw)))

        const proposalId  = (await contractGovernance.query.hashProposal(transactions, descriptionHash)).value.unwrapRecursively().unwrapRecursively()

        await contractGovernance.tx.propose(transactions,'#proposer=' + Uint8ArrayToString(deployer.addressRaw))

        const proposalSnapshot = (await contractGovernance.query.proposalSnapshot(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalSnapshot)

        await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.fulfilled

        const proposalDeadline = (await contractGovernance.query.proposalDeadline(proposalId)).value.unwrapRecursively().unwrapRecursively()
        await contractGovernance.tx.setBlockTimestamp(proposalDeadline)

        await expect(contractGovernance.tx.execute(transactions, descriptionHash)).to.eventually.be.fulfilled

        await expect(contractGovernance.tx.cancel(transactions, descriptionHash)).to.eventually.be.rejected

        await api.disconnect()
      })
    })
  })

  describe('proposal length', function () {
    it('empty', async function () {
    //
    })

    it('mismatch #1', async function () {
    //
    })

    it('mismatch #2', async function () {
      //
    })

    it('mismatch #3', async function () {
      //
    })
  })

  describe('frontrun protection using description suffix', function () {
    describe('without protection', function () {
      describe('without suffix', function () {
        it('proposer can propose', async function () {
          //
        })

        it('someone else can propose', async function () {
        //
        })
      })

      describe('with different suffix', function () {
        beforeEach(async function () {
          //
        })

        it('proposer can propose', async function () {
          //
        })

        it('someone else can propose', async function () {
          //
        })
      })

      describe('with proposer suffix but bad address part', function () {
        beforeEach(async function () {
          //
        })

        it('propose can propose', async function () {
          //
        })

        it('someone else can propose', async function () {
          //
        })
      })
    })

    describe('with protection via proposer suffix', function () {
      beforeEach(async function () {
        //
      })

      it('proposer can propose', async function () {
        //
      })

      it('someone else cannot propose', async function () {
        //
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
      const {api, contractGovernance} = await setup()

      const proposal: Transaction = {
        callee: contractGovernance.address,
        selector: getSelectorByName(contractGovernance.abi.messages, 'setVotingDelay'),
        destination: contractGovernance.address,
        input: [100],
        transferredValue: 0,
        gasLimit: 1000000000000
      }


    })

    it('cannot setVotingPeriod to 0 through governance', async function () {
      //
    })

    it('can setProposalThreshold to 0 through governance', async function () {
      // 
    })
  })
})

  