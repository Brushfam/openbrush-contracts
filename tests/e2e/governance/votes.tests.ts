import {ApiPromise} from "@polkadot/api";
import {expect, getSigners} from "../helpers";
import ConstructorsVotes from "../../../typechain-generated/constructors/my_psp22_votes";
import ContractVotes from "../../../typechain-generated/contracts/my_psp22_votes";
import ConstructorsGovernance from "../../../typechain-generated/constructors/my_governor";
import ContractGovernance from "../../../typechain-generated/contracts/my_governor";
import {GovernorHelper} from "./helper";

describe('Votes', function () {

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

    return {
      api,
      alice,
      bob,
      deployer,
      contractVotes,
      contractAddressVotes,
    }
  }

  beforeEach(async function () {
    //
  })

  it('starts with zero votes', async function () {
    const {
      api,
      alice,
      bob,
      deployer,
      contractVotes,
      contractAddressVotes,
    } = await setup()

    await expect(contractVotes.query.totalSupply()).to.have.bnToNumber(0)

    await api.disconnect()
  })

  describe('performs voting operations', function () {
    beforeEach(async function () {
      //
    })

    it('reverts if block number >= current block', async function () {
      //
    })

    it('delegates', async function () {
      //
    })

    it('cross delegates', async function () {
      //
    })

    it('returns total amount of votes', async function () {
      //
    })
  })
})