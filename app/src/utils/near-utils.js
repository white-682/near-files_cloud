// import { Account, keyStores, Near, WalletConnection } from 'near-api-js'
import getConfig from '../config'
import * as nearAPI from 'near-api-js'
import {
  formatNearAmount,
  parseNearAmount
} from 'near-api-js/lib/utils/format'

const nearConfig = getConfig('development')
window.nearConfig = nearConfig

async function initContract () {
  // Initializing connection to the NEAR node.
  const near = await nearAPI.connect({
    deps: {
      keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore()
    },
    ...nearConfig
  })

  // Needed to access wallet login
  const walletAccount = new nearAPI.WalletConnection(near)

  // Initializing our contract APIs by contract name and configuration.
  const contract = await new nearAPI.Contract(
    walletAccount.account(),
    nearConfig.contractName,
    {
      viewMethods: ['view_account'],
      changeMethods: ['store', 'file_delete','folder_create','folder_rename','folder_delete','file_copy_to_folder','file_delete_in_folder'],
      sender: walletAccount.getAccountId()
    }
  )
  window.walletAccount = walletAccount
  window.contract = contract
}

const getBalance = async ({ wallet }) => {
  return formatNearAmount(
    (await wallet.account().getAccountBalance()).available,
    4
  )
}

export default {
  initContract,
  getBalance,
  parseNearAmount,
  nearConfig
}
