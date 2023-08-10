import { expect, getSigners } from '../helpers'
import { ApiPromise } from '@polkadot/api'

import Constructors from '../../../typechain-generated/constructors/nonces'
import Contract from '../../../typechain-generated/contracts/nonces'
import BN from 'bn.js'
import {KeyringPair} from '@polkadot/keyring/types'

describe('Nonces', function () {
  let api: ApiPromise
  let deployer: KeyringPair
  let bob: KeyringPair

  let contract : Contract

  async function setup() {
    api = await ApiPromise.create()

    const signers = getSigners()
    deployer = signers[0]
    bob = signers[1]

    const contractFactory = new Constructors(api, deployer)
    const contractAddress = (await contractFactory.new()).address

    contract = new Contract(contractAddress, deployer, api)
  }

  beforeEach(async function () {
    await setup()
  })

  afterEach(async function () {
    await api.disconnect()
  })

  it('gets a nonce', async function () {
    const nonce = await contract.query.nonces(bob.address)
    await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
    await api.disconnect()
  })

  describe('_useNonce', function () {
    it('increments a nonce', async function () {
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
      await contract.tx.useNonce(bob.address)
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(1)
      await contract.tx.useNonce(bob.address)
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(2)
      await api.disconnect()
    })

  })

  describe('_useCheckedNonce', function () {
    it('increments a nonce', async function () {
      await expect(contract.query.useCheckedNonce(bob.address, new BN(0))).to.have.bnToNumber(0)
      await contract.withSigner(bob).tx.useCheckedNonce(bob.address, new BN(0))
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(1)
      await expect(contract.query.useCheckedNonce(bob.address, new BN(1))).to.have.bnToNumber(1)
    })

    it('reverts when nonce is not the expected', async function () {
      await expect(contract.query.nonces(bob.address)).to.have.bnToNumber(0)
      await expect(contract.withSigner(bob).tx.useCheckedNonce(bob.address, new BN(1))).to.eventually.be.rejected
      await expect(contract.tx.useCheckedNonce(bob.address, new BN(0))).to.eventually.be.fulfilled
    })
  })
})