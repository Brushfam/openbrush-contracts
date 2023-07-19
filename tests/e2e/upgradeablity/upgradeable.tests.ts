import { expect, getSigners } from '../helpers'
import { ApiPromise } from '@polkadot/api'


import ConstructorsV1 from '../../../typechain-generated/constructors/contract_v1'
import ContractV1 from '../../../typechain-generated/contracts/contract_v1'

import ConstructorsV2 from '../../../typechain-generated/constructors/contract_v2'
import ContractV2 from '../../../typechain-generated/contracts/contract_v2'

describe('UPGRADEABLE', () => {
  async function setup() {
    const api = await ApiPromise.create()
    const initialAmount = 1000000000000

    const signers = getSigners()
    const deployer = signers[0]
    const bob = signers[1]
    const feeCollector = signers[2]

    const contractFactory = new ConstructorsV1(api, deployer)
    const contractAddress = (await contractFactory.new(initialAmount)).address
    const contractV1 = new ContractV1(contractAddress, deployer, api)

    const contractFactoryV2 = new ConstructorsV2(api, deployer)
    await contractFactoryV2.new(0)
    const contractV2 = new ContractV2(contractAddress, deployer, api)

    

    return {
      codeHash: Array.from(contractV2.abi.info.source.wasmHash),
      api,
      bob,
      feeCollector,
      initialAmount,
      deployer,
      contractV1,
      contractV2
    }
  }

  it('Upgrade with set_code_hash', async () => {
    const { codeHash, api, contractV1, contractV2, deployer, bob, feeCollector, initialAmount } = await setup()

    const transferAmount = 1000

    // Arrange - create V1, transfer to bob
    await setup()

    await contractV1.withSigner(deployer).tx.transfer(bob.address, transferAmount, [])
    
    // Act - upgrade to v2, set fee collector, transfer to bob
    // upgrade to v2
    await contractV1.withSigner(deployer).tx.setCodeHash(codeHash)
    // first transfer is without fee since we did not set fee receiver
    await contractV2.withSigner(deployer).tx.transfer(bob.address, transferAmount, [])
    // set fee receiver
    await contractV2.withSigner(deployer).tx.setFeeCollector(feeCollector.address)
    // now the transfer is with fee
    await contractV2.withSigner(deployer).tx.transfer(bob.address, transferAmount, [])

    // Assert - check if correct amounts
    const balanceDeployer = (await contractV2.query.balanceOf(deployer.address)).value.unwrapRecursively().toNumber()
    const balanceBob = (await contractV2.query.balanceOf(bob.address)).value.unwrapRecursively().toNumber()
    const balanceFeeColletor = (await contractV2.query.balanceOf(feeCollector.address)).value.unwrapRecursively().toNumber()

    expect(balanceDeployer).to.be.eq(initialAmount - transferAmount * 3)
    expect(balanceBob).to.be.eq(transferAmount * 3 - transferAmount / 10)
    expect(balanceFeeColletor).to.be.eq(transferAmount / 10)

    await api.disconnect()
  })
})
