import BN from 'bn.js'
import {Transaction, VoteType} from '../../../typechain-generated/types-arguments/my_governor'
import ContractGovernance from '../../../typechain-generated/contracts/my_governor'
import {KeyringPair} from '@polkadot/keyring/types'
import {blake2AsU8a} from '@polkadot/util-crypto'
import ContractVotes from '../../../typechain-generated/contracts/my_psp22_votes'
import {hexToNumbers} from '../helpers'

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

  paramsToInput(params: Uint8Array) {
    let ecdStr = ''
    for (let i = 1; i < params.length; ++i) {
      let stemp = params[i].toString(16)
      if (stemp.length < 2) {
        stemp = '0' + stemp
      }
      ecdStr += stemp
    }
    const selector = hexToNumbers(ecdStr.substring(0, 8))
    const data = hexToNumbers(ecdStr.substring(8))
    return { selector, data }
  }

  addProposal(callee: string, selector: number[], input: (string | number | BN)[], description: string) {
    this.proposal = {
      callee: callee,
      selector: selector,
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
    await this.token?.tx.setBlockTimestamp(proposalSnapshot + offset)
  }

  async increaseBlockTimestamp(timestamp = 0) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    await this.governor?.tx.increaseBlockTimestamp(timestamp)
    await this.token?.tx.increaseBlockTimestamp(timestamp)
  }

  async castVote(voter: KeyringPair, vote: VoteType) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    await this.governor?.withSigner(voter).tx.castVote(this.proposalId as unknown as number[], vote, null, null)
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

  async state() {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    return (await this.governor?.query.state(this.proposalId as unknown as number[]))?.value.ok?.ok
  }

  async hasVoted(voter: KeyringPair) {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    return (await this.governor?.query.hasVoted(this.proposalId as unknown as number[], voter.address))?.value.ok
  }

  async proposalVotes(): Promise<string> {
    if (this.proposal === undefined || this.description === undefined){
      throw new Error('Proposal not set')
    }

    if(this.proposalId === undefined) {
      this.proposalId = await this.getProposalId()
    }

    const votes = (await this.governor?.query.proposalVotes(this.proposalId as unknown as number[]))?.value.ok!.ok!
    const votesArr = Array.from([votes.forVotes, votes.againstVotes, votes.abstainVotes])
    return votesArr.toString()
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