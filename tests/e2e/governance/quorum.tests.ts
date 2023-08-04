import Constructor from '../../../typechain-generated/constructors/my_governor'
import Contract from '../../../typechain-generated/contracts/my_governor'
import {ApiPromise} from '@polkadot/api'
import {KeyringPair} from '@polkadot/keyring/types'
import {expect, getSigners} from '../helpers'


describe('QuorumImpl', () => {
  let api: ApiPromise
  let deployer: KeyringPair
  let bob: KeyringPair
  const tokenAddress = '5HrN7fHLXWcFiXPwwtq2EkSGns9eMt5P7SpeTPewumZy6ftb'
  let contract : Contract

  const VOTING_DELAY = 1000
  const VOTING_PERIOD = 1000
  const PROPOSAL_THRESHOLD = 1000
  const QUORUM_NUMERATOR = 100


  async function setup() {
    api = await ApiPromise.create()

    const signers = getSigners()
    deployer = signers[0]
    bob = signers[1]

    const contractFactory = new Constructor(api, deployer)
    const contractAddress = (await contractFactory.new(
      tokenAddress,
      VOTING_DELAY,
      VOTING_PERIOD,
      PROPOSAL_THRESHOLD,
      QUORUM_NUMERATOR
    )).address

    contract = new Contract(contractAddress, deployer, api)
  }

  beforeEach(async function () {
    await setup()
  })

  afterEach(async function () {
    await api.disconnect()
  })

  it('should return the initialized value as quorum numerator', async () => {
    await expect(contract.query.quorumNumerator()).to.have.bnToNumber(QUORUM_NUMERATOR)
  })

  it('should return the quorum numerator at a specific time point', async () => {
    await expect(contract.query.quorumNumeratorAt(0)).to.have.bnToNumber(0)
    
    const current_time = Date.now()

    await expect(contract.query.quorumNumeratorAt(current_time)).to.have.bnToNumber(QUORUM_NUMERATOR)
  })

  it('should calculate quorum with valid numerator and denominator', async () => {
    /// TODO: finish after tests for PSP22Votes


    // let current_time = Date.now()
    //
    // await expect(contract.query.quorum(current_time)).to.have.bnToNumber(QUORUM_NUMERATOR)

    // await contract.tx.updateQuorumNumerator(QUORUM_NUMERATOR / 2)

    // current_time = Date.now()

    // await expect(contract.query.quorum(current_time)).to.have.bnToNumber(QUORUM_NUMERATOR / 2)
  })

  it('should update quorum numerator with a valid value', async () => {
    await expect(contract.query.quorumNumerator()).to.have.bnToNumber(QUORUM_NUMERATOR)

    await contract.tx.updateQuorumNumerator(QUORUM_NUMERATOR / 2)

    await expect(contract.query.quorumNumerator()).to.have.bnToNumber(QUORUM_NUMERATOR / 2)
  })

  it('should not update quorum numerator with a value greater than denominator', async () => {
    await expect(contract.query.quorumNumerator()).to.have.bnToNumber(QUORUM_NUMERATOR)

    await expect(contract.tx.updateQuorumNumerator(QUORUM_NUMERATOR * 2)).to.be.rejected

    await expect(contract.query.quorumNumerator()).to.have.bnToNumber(QUORUM_NUMERATOR)
  })
})
