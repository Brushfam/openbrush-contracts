import { expect, getSigners } from '../helpers'
import { ApiPromise } from '@polkadot/api'

import Constructor from '../../../typechain-generated/constructors/nonces'
import Contract from '../../../typechain-generated/contracts/nonces'
import BN from "bn.js";
import {Result} from "@727-ventures/typechain-types";

describe('Nonces', function () {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const deployer = signers[0]
    const bob = signers[1]

    const contractFactory = new Constructor(api, deployer)
    const contractAddress = (await contractFactory.new()).address
    const contract = new Contract(contractAddress, deployer, api)

    return {
      api,
      bob,
      deployer,
      contract,
    }
  }

  beforeEach(async function () {
    //
  })

  it('gets a nonce', async function () {
    const {
      api,
      bob,
      deployer,
      contract,
    } = await setup()
    const nonce = await contract.query.nonces(bob.address)
    await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
    await api.disconnect()
  })

  describe('_useNonce', function () {
    it('increments a nonce', async function () {
      const {
        api,
        bob,
        deployer,
        contract,
      } = await setup()
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
      await contract.tx.useNonce(bob.address)
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(1)
      await contract.tx.useNonce(bob.address)
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(2)
      await api.disconnect()
    })

    // it('increments only sender\'s nonce', async function () {
    //   //
    // })
  })

  describe('_useCheckedNonce', function () {
    it('increments a nonce', async function () {
      const {
        api,
        bob,
        deployer,
        contract,
      } = await setup()
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
      await expect(contract.query.useCheckedNonce(bob.address, new BN(0))).to.have.bnToNumber(0)
      await contract.withSigner(bob).tx.useCheckedNonce(bob.address, new BN(0))
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(1)
      await expect(contract.query.useCheckedNonce(bob.address, new BN(1))).to.have.bnToNumber(1)
    })

    // it('increments only sender\'s nonce', async function () {
    //   //
    // })

    it('reverts when nonce is not the expected', async function () {
      const {
        api,
        bob,
        deployer,
        contract,
      } = await setup()
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
      await expect(contract.withSigner(bob).tx.useCheckedNonce(bob.address, new BN(1))).to.eventually.be.rejected
      await expect(contract.tx.useCheckedNonce(bob.address, new BN(0))).to.eventually.be.fulfilled

    })
  })
})