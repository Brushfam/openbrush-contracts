import {ApiPromise} from "@polkadot/api";
import {getSigners} from "../helpers";
import ConstructorsGovernance from "../../../typechain-generated/constructors/my_governor";
import ContractGovernance from "../../../typechain-generated/contracts/my_governor";
import type * as ArgumentTypes from '../../../typechain-generated/types-arguments/my_governor';
import {VoteType} from "../../../typechain-generated/types-arguments/my_governor";

import ConstructorsVotes from "../../../typechain-generated/constructors/my_psp22_votes";
import ContractVotes from "../../../typechain-generated/contracts/my_psp22_votes";
import BN from "bn.js";
import {expect} from "chai";
import {AbiMessage} from "@polkadot/api-contract/types";
import {contractsAbi} from "@polkadot/types/interfaces/definitions";
import {str} from "@scure/base";

describe('Governor', function () {

  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const deployer = signers[0]
    const bob = signers[1]

    const totalSupply = 100000
    const contractFactoryVotes = new ConstructorsVotes(api, deployer)
    const contractAddressVotes = (await contractFactoryVotes.new(totalSupply)).address
    const contractVotes = new ContractVotes(contractAddressVotes, deployer, api)

    const votingDelay = 0
    const votingPeriod = 10
    const proposalThreshold = 0
    const numratom = 0
    const contractFactoryGovernance = new ConstructorsGovernance(api, deployer)
    const contractAddressGovernance = (await contractFactoryGovernance.new(contractAddressVotes, 0, 10, 0, 0)).address
    const contractGovernance = new ContractGovernance(contractAddressGovernance, deployer, api)

    return {
      api,
      bob,
      deployer,
      contractGovernance,
      contractVotes,
    }
  }

  const getSelectorsFromMessages = (messages: AbiMessage[]): number[][] => {
    return messages.map((message) => {
      return message.selector.toU8a() as unknown as number[]
    })
  }

  const getSelectorByName = (messages: AbiMessage[], name: string): number[] => {
    return messages.filter((message) => {
      return message.identifier == name
    })[0].selector.toU8a() as unknown as number[]
  }

  const Uint8ArrayToString = (array : Uint8Array) : string => {
    let res : string = '['
    for (let i = 0; i < array.length; i++) {
        res += array[i]
        if (i != array.length - 1) {
            res += ', '
        }
    }
    res += ']'
    return res
  }
  
  beforeEach(async function () {
  //   
  })
    
  it('deployment check', async function () {
    const {
      api,
      bob,
      deployer,
      contractGovernance,
      contractVotes,
    } = await setup()
    api.disconnect()
  })

  it('nominal workflow', async function () {
    //
  })

  it('send ethers', async function () {
    //
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
        let transactions: Array<ArgumentTypes.Transaction> = [{
          callee: contractVotes.address,
          selector: getSelectorByName(contractVotes.abi.messages, 'PSP22::transfer'),
          destination: contractVotes.address,
          input: [bob.address, new BN(1000), ''], // [to, value, data]
          transferredValue: 0,
          gasLimit: 1000000000000,
        }]
        console.log("#proposer=" + Uint8ArrayToString(deployer.addressRaw))
        await expect(contractGovernance.query.propose(transactions,"#proposer=" + Uint8ArrayToString(deployer.addressRaw))).to.have.bnToString('0')
        await expect(contractGovernance.tx.propose(transactions,"#proposer=" + Uint8ArrayToString(deployer.addressRaw))).to.eventually.be.rejected
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
          contractVotes,
        } = await setup()
        await expect(contractGovernance.tx.castVote([21], VoteType.for)).to.eventually.be.rejected
      })

      it('if voting has not started', async function () {
      //
      })

      it('if support value is invalid', async function () {
      //
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
      //
    })

    it('setVotingPeriod is protected', async function () {
      //
    })

    it('setProposalThreshold is protected', async function () {
      //
    })

    it('can setVotingDelay through governance', async function () {
      // 
    })

    it('cannot setVotingPeriod to 0 through governance', async function () {
      //
    })

    it('can setProposalThreshold to 0 through governance', async function () {
      // 
    })
  })
})

  