import BN from 'bn.js'
import {Transaction, VoteType} from '../../../typechain-generated/types-arguments/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import {KeyringPair} from '@polkadot/keyring/types'
import {blake2AsU8a} from '@polkadot/util-crypto'
import {ProposalState} from "../../../typechain-generated/types-returns/my_governor";
import ContractVotes from "../../../typechain-generated/contracts/my_psp22_votes"

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

  async getProposalId() {
    if (this.proposal === undefined || this.description === undefined) {
      throw new Error('Proposal not set')
    }

    this.proposalId = (await this.governor?.query.propose([this.proposal!], this.description!))?.value.ok!.ok

    return this.proposalId
  }

  async delegate(token: ContractVotes, from: KeyringPair, to: KeyringPair, amount: number) {
    await token.withSigner(from).tx.transfer(to.address, amount, [])
    await token.withSigner(to).tx.delegate(to.address)

    /*console.log('delegate to' +  to.address)

    console.log((await token.query.getVotes(to.address)).value.ok?.ok?.toNumber())
    console.log((await token.query.balanceOf(to.address)).value.unwrapRecursively().toNumber())*/
  }

  async propose(proposer?: KeyringPair) {
    if (this.proposal === undefined || this.description === undefined) {
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    if(proposer) {
      await this.governor?.withSigner(proposer).tx.propose([this.proposal!], this.description!)
    }
    else {
      await this.governor?.tx.propose([this.proposal!], this.description!)
    }
  }

  async waitForSnapshot(offset = 0) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    const proposalSnapshot = (await this.governor?.query.proposalSnapshot(this.proposalId as unknown as number[]))?.value.ok!.ok

    if(proposalSnapshot === undefined) throw new Error('Proposal snapshot not set')

    await this.governor?.tx.setBlockTimestamp(proposalSnapshot + offset)
  }

  async castVote(voter: KeyringPair, vote: VoteType) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    await this.governor?.withSigner(voter).tx.castVote(this.proposalId as unknown as number[], vote)

    console.log((await this.governor?.query.proposalVotes(await this.proposalId as unknown as number[]))?.value.ok!.ok)
  }

  async waitForDeadline(offset = 0) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    const proposalDeadline = (await this.governor?.query.proposalDeadline(this.proposalId as unknown as number[]))?.value.ok!.ok

    if(proposalDeadline === undefined) throw new Error('Proposal deadline not set')

    await this.governor?.tx.setBlockTimestamp(proposalDeadline + offset)
  }

  async execute(proposer?: KeyringPair) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
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
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    const descriptionHash = blake2AsU8a(this.description!) as unknown as number[]

    if (proposer) {
      await this.governor?.withSigner(proposer).tx.cancel([this.proposal!], descriptionHash)
    }
    else{
      await this.governor?.tx.cancel([this.proposal!], descriptionHash)
    }
  }

  async state(): Promise<ProposalState> {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    console.log((await this.governor?.query.state(this.proposalId as unknown as number[]))?.value)

    return (await this.governor?.query.state(this.proposalId as unknown as number[]))?.value.ok!.ok!
  }
}