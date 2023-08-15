import {ApiPromise} from "@polkadot/api";
import {expect, getSigners} from "../helpers";
import ConstructorsVotes from "../../../typechain-generated/constructors/my_psp22_votes";
import ContractVotes from "../../../typechain-generated/contracts/my_psp22_votes";
import ConstructorsGovernance from "../../../typechain-generated/constructors/my_governor";
import ContractGovernance from "../../../typechain-generated/contracts/my_governor";
import {GovernorHelper} from "./helper";
import {q} from "@noble/curves/pasta";
import {consts} from "../constants";

const ZERO_ADDRESS = '5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM'
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
    const account1 = signers[1]
    const account2 = signers[2]
    const account3 = signers[3]
    const contractFactoryVotes = new ConstructorsVotes(api, deployer)
    const contractAddressVotes = (await contractFactoryVotes.new(totalSupply)).address
    const contractVotes = new ContractVotes(contractAddressVotes, deployer, api)

    return {
      api,
      account1,
      account2,
      account3,
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
      contractVotes,
    } = await setup(0)

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
      const {
        api,
        account1,
        account2,
        account3,
        deployer,
        contractVotes,
        contractAddressVotes,
      } = await setup()

      const amounts = {
        account1: 100000,
        account2: 10,
        account0: 20
      };

      await contractVotes.tx.mint(deployer.address, amounts.account0)
      await contractVotes.tx.mint(account1.address, amounts.account1)
      await contractVotes.tx.mint(account2.address, amounts.account2)

      await expect((await (contractVotes.query.getVotes(account1.address))).value.ok!.ok!.rawNumber.toNumber()).to.equals(0)
      await expect((await contractVotes.query.getVotes(account2.address)).value.ok!.ok!.rawNumber.toNumber()).to.equals(0)
      await expect((await contractVotes.query.delegates(account1.address)).value.ok!).to.equals(null)
      await expect((await contractVotes.query.delegates(account2.address)).value.ok!).to.equals(null)

      await contractVotes.withSigner(account1).tx.delegate(account1.address)

      await expect((await (contractVotes.query.getVotes(account1.address))).value.ok!.ok!.rawNumber.toNumber()).to.equals(amounts.account1)
      await expect((await contractVotes.query.getVotes(account2.address)).value.ok!.ok!.rawNumber.toNumber()).to.equals(0)
      await expect((await contractVotes.query.delegates(account1.address)).value.ok!).to.equals(account1.address)
      await expect((await contractVotes.query.delegates(account2.address)).value.ok!).to.equals(null)

      await contractVotes.withSigner(account2).tx.delegate(account1.address)

      await expect((await (contractVotes.query.getVotes(account1.address))).value.ok!.ok!.rawNumber.toNumber()).to.equals(amounts.account1 + amounts.account2)
      await expect((await contractVotes.query.getVotes(account2.address)).value.ok!.ok!.rawNumber.toNumber()).to.equals(0)
      await expect((await contractVotes.query.delegates(account1.address)).value.ok!).to.equals(account1.address)
      await expect((await contractVotes.query.delegates(account2.address)).value.ok!).to.equals(account1.address)

      await contractVotes.withSigner(account1).tx.delegate(account2.address)
      await contractVotes.withSigner(account2).tx.delegate(account2.address)

      await expect((await (contractVotes.query.getVotes(account1.address))).value.ok!.ok!.rawNumber.toNumber()).to.equals(0)
      await expect((await contractVotes.query.getVotes(account2.address)).value.ok!.ok!.rawNumber.toNumber()).to.equals(amounts.account1 + amounts.account2)
      await expect((await contractVotes.query.delegates(account1.address)).value.ok!).to.equals(account2.address)
      await expect((await contractVotes.query.delegates(account2.address)).value.ok!).to.equals(account2.address)

      await api.disconnect()
    })

    it('cross delegates', async function () {
      const {
        api,
        account1,
        account2,
        account3,
        deployer,
        contractVotes,
        contractAddressVotes,
      } = await setup(0)

      const amounts = {
        account1: 100000,
        account2: 10,
        account0: 20
      };

      await contractVotes.tx.mint(deployer.address, amounts.account0)
      await contractVotes.tx.mint(account1.address, amounts.account1)
      await contractVotes.tx.mint(account2.address, amounts.account2)

      await contractVotes.withSigner(account1).tx.delegate(account2.address)
      await contractVotes.withSigner(account2).tx.delegate(account1.address)

      await expect(contractVotes.query.getVotes(account1.address)).to.have.bnToNumber(amounts.account2)
      await expect(contractVotes.query.getVotes(account2.address)).to.have.bnToNumber(amounts.account1)

      await api.disconnect()
    })

    it('returns total amount of votes', async function () {
      const {
        api,
        account1,
        account2,
        account3,
        deployer,
        contractVotes,
        contractAddressVotes,
      } = await setup(0)

      const amounts = {
        account1: 100000,
        account2: 10,
        account0: 20
      };

      const totalSupply = Object.values(amounts).reduce((a, b) => a + b, 0)

      await contractVotes.tx.mint(deployer.address, amounts.account0)
      await contractVotes.tx.mint(account1.address, amounts.account1)
      await contractVotes.tx.mint(account2.address, amounts.account2)
      // await contractVotes.tx.mint(account3.address, amounts.account3)

      await expect(contractVotes.query.totalSupply()).to.have.bnToNumber(totalSupply)

      await api.disconnect()
    })
  })
})