import BN from 'bn.js'
import {Transaction, VoteType} from '../../../typechain-generated/types-arguments/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import {KeyringPair} from '@polkadot/keyring/types'
import {blake2AsU8a} from '@polkadot/util-crypto'
import {ProposalState} from "../../../typechain-generated/types-returns/my_governor";
import ContractVotes from "../../../typechain-generated/contracts/my_psp22_votes"
import {ReturnNumber} from "@727-ventures/typechain-types";
import {bool} from "@polkadot/types-codec";

export class GovernorHelper {
  private proposal: Transaction | undefined;
  private description: string | undefined;
  private governor: ContractGovernance | undefined;
  private token: ContractVotes | undefined;
  private proposalId: number[] | undefined;

  constructor(governor: ContractGovernance, token: ContractVotes){
    this.governor = governor
    this.token = token
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
      console.log((await this.governor?.query.propose([this.proposal!], this.description!))?.value)
      await this.governor?.withSigner(proposer).tx.propose([this.proposal!], this.description!)
    }
    else {
      console.log((await this.governor?.query.propose([this.proposal!], this.description!))?.value)
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
    await this.token?.tx.setBlockTimestamp(proposalSnapshot + offset)
  }

  async castVote(voter: KeyringPair, vote: VoteType) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    await this.governor?.withSigner(voter).tx.castVote(this.proposalId as unknown as number[], vote)

    // console.log((await this.governor?.query.proposalVotes(await this.proposalId as unknown as number[]))?.value.ok!.ok)
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
    await this.token?.tx.setBlockTimestamp(proposalDeadline + offset)
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

    // console.log((await this.governor?.query.state(this.proposalId as unknown as number[]))?.value)

    return (await this.governor?.query.state(this.proposalId as unknown as number[]))?.value.ok!.ok!
  }

  async hasVoted(voter: KeyringPair): Promise<boolean> {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    console.log('has voted')
    console.log((await this.governor?.query.hasVoted(this.proposalId as unknown as number[], voter.address))?.value.ok!)
    return (await this.governor?.query.hasVoted(this.proposalId as unknown as number[], voter.address))?.value.ok!
  }

  async proposalVotes(): Promise<number[]> {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    const votes = (await this.governor?.query.proposalVotes(this.proposalId as unknown as number[]))?.value.ok!.ok!
    return [votes[0].rawNumber.toNumber(), votes[1].rawNumber.toNumber(), votes[2].rawNumber.toNumber()]
  }

  async getVotes(voter: KeyringPair): Promise<number|undefined> {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    const proposalSnapshot = (await this.governor?.query.proposalSnapshot(this.proposalId as unknown as number[]))?.value.ok!.ok

    if(proposalSnapshot === undefined) throw new Error('Proposal snapshot not set')

    return (await this.governor?.query.getVotes(voter.address, proposalSnapshot, []))?.value.ok!.ok!.toNumber()
  }

  async countVotes(voter: KeyringPair, support: VoteType,  weight: number): Promise<void> {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    await this.governor?.tx.countVote(this.proposalId as unknown as number[], voter.address, support,  weight)
  }
}