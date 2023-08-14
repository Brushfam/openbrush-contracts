import BN from 'bn.js'
import { expect } from './setup/chai'
import { KeyringPair } from '@polkadot/keyring/types'
import { Keyring } from '@polkadot/keyring'
import {AbiMessage} from "@polkadot/api-contract/types";
import {ApiPromise} from "@polkadot/api";

export { expect } from './setup/chai'

export const bnArg = (value: number | string | number[] | Uint8Array | Buffer | BN, len = 32) => {
  return new BN(value, undefined, 'le').toArray('le', len)
}

export const oneDay = () => (24 * 60 * 60 * 1000)

export const getSigners = (): KeyringPair[] => {
  const keyring = new Keyring({type: 'sr25519'})

  const UserAlice: KeyringPair = keyring.addFromUri('//Alice')
  const UserBob: KeyringPair = keyring.addFromUri('//Bob')
  const UserCharlie: KeyringPair = keyring.addFromUri('//Charlie')

  return [
    UserAlice, UserBob, UserCharlie
  ]
}

export function bytesToString(bytes: string): string {
  const outputNumber = bytes.substring(2).split('').map(x => parseInt(x as unknown as string, 16))

  const length = outputNumber.length
  let result = ''
  for (let i = 0; i < length; i += 2) {
    result += String.fromCharCode(outputNumber[i] * 16 + outputNumber[i + 1])
  }

  return result
}

export const getSelectorsFromMessages = (messages: AbiMessage[]): number[][] => {
  return messages.map((message) => {
    return message.selector.toU8a() as unknown as number[]
  })
}

export const getSelectorByName = (messages: AbiMessage[], name: string): number[] => {
  return messages.filter((message) => {
    return message.identifier.includes(name)
  })[0].selector.toU8a() as unknown as number[]
}

export const Uint8ArrayToString = (array : Uint8Array) : string => {
  let res  = '['
  for (let i = 0; i < array.length; i++) {
    res += array[i]
    if (i != array.length - 1) {
      res += ', '
    }
  }
  res += ']'
  return res
}

export const SS58ToHex = (api: ApiPromise, ss58: string): string => {
  return api.registry.createType('AccountId', ss58).toHex()
}