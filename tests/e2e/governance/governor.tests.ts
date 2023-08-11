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
import {AbiMessage} from '@polkadot/api-contract/types'
import {contractsAbi} from '@polkadot/types/interfaces/definitions'
import {str} from '@scure/base'

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
      api
    } = await setup()
    api.disconnect()
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

        api.disconnect()
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
        api.disconnect()
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
        api.disconnect()
      })

      it('if support value is invalid', async function () {
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
        // await expect(contractGovernance.tx.castVote(proposalId, VoteType.for)).to.eventually.be.rejected
        api.disconnect()
      })

      it('if vote was already casted', async function () {
        //
      })

      it('if voting is over', async function () {
        //
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
        //
      })

      it('if quorum is not reached', async function () {
        //
      })

      it('if score not reached', async function () {
        //
      })

      it('if voting is not over', async function () {
        //
      })

      it('if receiver revert without reason', async function () {
        //
      })

      it('if receiver revert with reason', async function () {
        //
      })

      it('if proposal was already executed', async function () {
        //
      })
    })
  })

  describe('state', function () {
    it('Unset', async function () {
      //
    })

    it('Pending & Active', async function () {
      //
    })

    it('Defeated', async function () {
      //
    })

    it('Succeeded', async function () {
      //
    })

    it('Executed', async function () {
      //
    })
  })

  describe('cancel', function () {
    describe('internal', function () {
      it('before proposal', async function () {
        //
      })

      it('after proposal', async function () {
        //
      })

      it('after vote', async function () {
        //
      })

      it('after deadline', async function () {
        //
      })

      it('after execution', async function () {
        //
      })
    })

    describe('public', function () {
      it('before proposal', async function () {
        //
      })

      it('after proposal', async function () {
        //
      })

      it('after proposal - restricted to proposer', async function () {
        //
      })

      it('after vote started', async function () {
        //
      })

      it('after vote', async function () {
        //
      })

      it('after deadline', async function () {
        //
      })

      it('after execution', async function () {
        //
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

      api.disconnect()
    })

    it('setVotingPeriod is protected', async function () {
      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.setVotingPeriod(100)).to.eventually.be.rejected

      api.disconnect()
    })

    it('setProposalThreshold is protected', async function () {
      const {api, contractGovernance} = await setup()

      await expect(contractGovernance.tx.setProposalThreshold(100)).to.eventually.be.rejected

      api.disconnect()
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

  