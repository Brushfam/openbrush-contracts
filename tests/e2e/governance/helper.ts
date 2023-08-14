import BN from 'bn.js'
import {Transaction, VoteType} from '../../../typechain-generated/types-arguments/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import {KeyringPair} from '@polkadot/keyring/types'
import {blake2AsU8a} from '@polkadot/util-crypto'
import {ProposalState} from "../../../typechain-generated/types-returns/my_governor";

export class GovernorHelper {
  private proposal: Transaction | undefined;
  private description: string | undefined;
  private governor: ContractGovernance | undefined;
  private proposalId: number[] | undefined;

  constructor(governor: ContractGovernance) {
    this.governor = governor
  }

  addProposal(callee: string, selector: number[], input: (string | number | BN)[], description: string) {
    this.proposal = {
      callee: callee,
      selector: selector,
      destination: callee,
      input: input,
      transferredValue: 0,
      gasLimit: 1000000000000
    }
    this.description = description
  }
  
  getProposalId(): number[] | undefined {
    return this.proposalId
  }

  async calculateProposalId() {
    if (this.proposal === undefined || this.description === undefined) {
      throw new Error('Proposal not set')
    }

    const descriptionHash = blake2AsU8a(this.description!) as unknown as number[]

    const proposalId = (await this.governor?.query.hashProposal([this.proposal!], descriptionHash))?.value.ok!.ok

    return proposalId
  }

  async propose(proposer?: KeyringPair) {
    if (this.proposal === undefined || this.description === undefined) {
      throw new Error('Proposal not set')
    }

    console.log('Proposal ID: ', await this.calculateProposalId())

    if(proposer) {
      await this.governor?.withSigner(proposer).tx.propose([this.proposal!], this.description!)
    }
    else {
      await this.governor?.tx.propose([this.proposal!], this.description!)
    }
  }

  async waitForSnapshot(offset = 0) {
    const proposalSnapshot = (await this.governor?.query.proposalSnapshot(this.proposalId as unknown as number[]))?.value.unwrapRecursively().ok
    await this.governor?.tx.setBlockTimestamp(proposalSnapshot as number + offset)
  }

  async castVote(voter: KeyringPair, vote: VoteType) {
    if (this.proposalId === undefined) {
      throw new Error('Proposal Id not set')
    }
    await this.governor?.withSigner(voter).tx.castVote(this.proposalId, vote)
  }

  async waitForDeadline(offset = 0) {
    const proposalDeadline = (await this.governor?.query.proposalDeadline(this.proposalId as unknown as number[]))?.value.unwrapRecursively().ok
    await this.governor?.tx.setBlockTimestamp(proposalDeadline as number + offset)
  }

  async execute(proposer?: KeyringPair) {
    if (this.proposalId === undefined) {
      throw new Error('Proposal Id not set')
    }

    const descriptionHash = blake2AsU8a(this.description!) as unknown as number[]

    if (proposer) {
      await this.governor?.withSigner(proposer).tx.execute([this.proposal!], descriptionHash)
    }
    else {
      await this.governor?.tx.execute([this.proposal!], descriptionHash)
    }
  }

  async cancel(proposer?: KeyringPair) {
    if(this.proposalId === undefined) {
      throw new Error('Proposal Id not set')
    }

    const descriptionHash = blake2AsU8a(this.description!) as unknown as number[]

    if (proposer) {
      await this.governor?.withSigner(proposer).tx.cancel([this.proposal!], descriptionHash)
    }
    else{
      await this.governor?.tx.cancel([this.proposal!], descriptionHash)
    }
  }

  async state(proposer: KeyringPair): Promise<ProposalState> {
    return ProposalState.active
  }
}