"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[75263],{3905:(e,t,n)=>{n.d(t,{Zo:()=>d,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function s(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?s(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):s(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},s=Object.keys(e);for(a=0;a<s.length;a++)n=s[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var s=Object.getOwnPropertySymbols(e);for(a=0;a<s.length;a++)n=s[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var i=a.createContext({}),c=function(e){var t=a.useContext(i),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},d=function(e){var t=c(e.components);return a.createElement(i.Provider,{value:t},e.children)},u="mdxType",h={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},p=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,s=e.originalType,i=e.parentName,d=l(e,["components","mdxType","originalType","parentName"]),u=c(n),p=r,f=u["".concat(i,".").concat(p)]||u[p]||h[p]||s;return n?a.createElement(f,o(o({ref:t},d),{},{components:n})):a.createElement(f,o({ref:t},d))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var s=n.length,o=new Array(s);o[0]=p;var l={};for(var i in t)hasOwnProperty.call(t,i)&&(l[i]=t[i]);l.originalType=e,l[u]="string"==typeof e?e:r,o[1]=l;for(var c=2;c<s;c++)o[c]=n[c];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}p.displayName="MDXCreateElement"},81938:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>i,contentTitle:()=>o,default:()=>u,frontMatter:()=>s,metadata:()=>l,toc:()=>c});var a=n(87462),r=(n(67294),n(3905));const s={sidebar_position:10,title:"Notes about methods"},o=void 0,l={unversionedId:"smart-contracts/example/implementation",id:"smart-contracts/example/implementation",title:"Notes about methods",description:"In this section, we describe the implementation of the functions of our lending",source:"@site/docs/smart-contracts/example/implementation.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/implementation",permalink:"/next/smart-contracts/example/implementation",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/docs/smart-contracts/example/implementation.md",tags:[],version:"current",sidebarPosition:10,frontMatter:{sidebar_position:10,title:"Notes about methods"},sidebar:"tutorialSidebar",previous:{title:"Lending contract",permalink:"/next/smart-contracts/example/contract"},next:{title:"Deployment",permalink:"/next/deployment"}},i={},c=[{value:"Instantiating contracts",id:"instantiating-contracts",level:2},{value:"Simulating oracle",id:"simulating-oracle",level:2},{value:"Allowing assets",id:"allowing-assets",level:2},{value:"Lending assets",id:"lending-assets",level:2},{value:"Borrowing assets",id:"borrowing-assets",level:2}],d={toc:c};function u(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,a.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"In this section, we describe the implementation of the functions of our lending\ncontract."),(0,r.kt)("h2",{id:"instantiating-contracts"},"Instantiating contracts"),(0,r.kt)("p",null,"Each asset that we will accept to be lent will have two underlying tokens:\nthe shares token and the reserves token. The shares token will represent a\nuser's share of the lent asset which they can then withdraw and the reserves\ntoken will represent the amount of asset lent since we don't want to keep\ntrack of all addresses and amounts which have borrowed the assets. We will\nsimply take this amount from the total supply of the underlying reserve token.\nSo when we are accepting an asset for lending, we need to create a new token\ncontract for shares and for reserves. We will define an internal function for\nthis:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {\n    let code_hash = self.lending.shares_contract_code_hash;\n    let salt = (<Self as DefaultEnv>::env().block_timestamp(), contract_name).encode();\n    let hash = xxh32(&salt, 0).to_le_bytes();\n    \n    let contract =\n        SharesContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))\n            .endowment(0)\n            .code_hash(code_hash)\n            .salt_bytes(&hash[..4])\n            .instantiate()\n            .unwrap();\n    contract.to_account_id()\n}\n")),(0,r.kt)("p",null,"This function will instantiate our ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesContract")," contract and return\nthe ",(0,r.kt)("inlineCode",{parentName:"p"},"AccountId")," of the instantiated contract. We will call this function\nwhen allowing assets."),(0,r.kt)("h2",{id:"simulating-oracle"},"Simulating oracle"),(0,r.kt)("p",null,"As mentioned before, we will not be using a price oracle in our example,\nbut we will use our own simulated oracle. And by simulated we mean adding\nsome storage fields which hold the info about price of an asset and a function\nonly callable by the account with ",(0,r.kt)("inlineCode",{parentName:"p"},"MANAGER")," role, which will set the price of\nthe asset. For that we define these functions:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[modifiers(only_role(MANAGER))]\ndefault fn set_asset_price(\n    &mut self,\n    asset_in: AccountId,\n    asset_out: AccountId,\n    price: Balance,\n) -> Result<(), LendingError> {\n    set_asset_price(self, &asset_in, &asset_out, &price);\n    Ok(())\n}\n\n/// this internal function will be used to set price of `asset_in` when we deposit `asset_out`\n/// we are using this function in our example to simulate an oracle\npub fn set_asset_price<T>(instance: &mut T, asset_in: &AccountId, asset_out: &AccountId, price: &Balance)\nwhere\n    T: Storage<Data>,\n{\n    instance.data().asset_price.insert(&(asset_in, asset_out), price);\n}\n")),(0,r.kt)("h2",{id:"allowing-assets"},"Allowing assets"),(0,r.kt)("p",null,"If we just started lending and borrowing random assets or using random assets\nas collateral there would be chaos in our smart contract.\nRegarding lending, it would not be a big problem, since if somebody is\nwilling to borrow an asset, it would generate a profit for the lender.\nBut if we started accepting random assets as collateral, anyone could just\nthrow a random coin as collateral and then just for example rug pull it and\nalso keep the borrowed assets. Because of this we will only accept certain\nassets for lending and using as collateral. For an asset to be accepted, an\naccount with the ",(0,r.kt)("inlineCode",{parentName:"p"},"MANAGER")," role needs to allow it with the ",(0,r.kt)("inlineCode",{parentName:"p"},"allow_asset")," function.\nWe will use a modifier from OpenBrush, which serves similarly to Solidity's\nfunction modifiers. The function will look like this:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#[modifiers(only_role(MANAGER))]\ndefault fn allow_asset(&mut self, asset_address: AccountId) -> Result<(), LendingError> {\n    // we will ensure the asset is not accepted already\n    if self.is_accepted_lending(asset_address) {\n        return Err(LendingError::AssetSupported)\n    }\n\n    // instantiate the shares of the lended assets\n    let shares_address = self._instantiate_shares_contract("LendingShares", "LS");\n    // instantiate the reserves of the borrowed assets\n    let reserves_address = self._instantiate_shares_contract("LendingReserves", "LR");\n    // accept the asset and map shares and reserves to it\n\n    accept_lending(self, asset_address, shares_address, reserves_address);\n    Ok(())\n}\n')),(0,r.kt)("h2",{id:"lending-assets"},"Lending assets"),(0,r.kt)("p",null,"For lending the assets  we will use the function ",(0,r.kt)("inlineCode",{parentName:"p"},"lend_assets(asset_address, amount)"),",\nwhere ",(0,r.kt)("inlineCode",{parentName:"p"},"asset_address")," is the address of ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," we want to deposit and ",(0,r.kt)("inlineCode",{parentName:"p"},"amount"),"\nis the amount of asset deposited. Some checks need to be checked to assure the correct\nbehavior of our contract. The asset deposited needs to be recognized by our contract\n(manager must have approved it). If it is not accepted, an error will be returned.\nThen the user must have approved the asset to spent by our contract and the user's\nbalance must be greater than or equal to ",(0,r.kt)("inlineCode",{parentName:"p"},"amount"),". So we will transfer the asset from\nthe user to the contract, mint shares to the user. To perform a cross contract call\nwe will be using the references to contracts ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesRef"),".\nWe will also add ",(0,r.kt)("inlineCode",{parentName:"p"},"when_not_paused")," modifier to this function,\nso it can be only called when the contract is not paused.\nThe code will look like this:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[modifiers(when_not_paused)]\ndefault fn lend_assets(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {\n    // we will be using these often so we store them in variables\n    let lender = Self::env().caller();\n    let contract = Self::env().account_id();\n    // ensure the user gave allowance to the contract\n    if PSP22Ref::allowance(&asset_address, lender, contract) < amount {\n        return Err(LendingError::InsufficientAllowanceToLend)\n    }\n    // ensure the user has enough assets\n    if PSP22Ref::balance_of(&asset_address, lender) < amount {\n        return Err(LendingError::InsufficientBalanceToLend)\n    }\n    // how much assets is already in the contract\n    // if the asset is not accepted by the contract, this function will return an error\n    let total_asset = self.total_asset(asset_address)?;\n    // transfer the assets from user to the contract|\n    PSP22Ref::transfer_from_builder(&asset_address, lender, contract, amount, Vec::<u8>::new())\n        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))\n        .fire()\n        .unwrap()?;\n    // if no assets were deposited yet we will mint the same amount of shares as deposited `amount`\n    let new_shares = if total_asset == 0 {\n        amount\n    } else {\n        // else we calculate how much shares will belong us after depositing the `amount`\n        (amount * self.total_shares(asset_address)?) / total_asset\n    };\n    let reserve_asset = get_reserve_asset(self, &asset_address)?;\n    // mint the shares token to the user\n    SharesRef::mint(&reserve_asset, lender, new_shares)?;\n    Ok(())\n}\n")),(0,r.kt)("h2",{id:"borrowing-assets"},"Borrowing assets"),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"borrow_assets(asset_address, collateral_address, amount)")," function will\nserve for the users to borrow assets from the smart contract.\n",(0,r.kt)("inlineCode",{parentName:"p"},"asset_address")," is the account id of the asset we want to borrow,\n",(0,r.kt)("inlineCode",{parentName:"p"},"collateral_address")," is the account id of asset which the user wants\nto use as collateral, and ",(0,r.kt)("inlineCode",{parentName:"p"},"amount")," is the amount of collateral deposited.\nOur contract will calculate the value of the deposited collateral and\nwill give the borrower 70% of the collateral value. For pricing, we would\nuse an oracle, but in this example, we will use our 'simulated oracle' -\nwe will just store the price info in our contract and the admin will\nbe able to change it. The liquidation price of the loan will be calculated\nat 75% of the collateral value. First of all the contract must not be paused,\nfor which we use modifier ",(0,r.kt)("inlineCode",{parentName:"p"},"when_not_paused"),". After that, for the borrowing\nto succeed, the ",(0,r.kt)("inlineCode",{parentName:"p"},"collateral_address")," must be accepted by the contract,\nthe contract needs to have enough allowance to spend the borrower's collateral\ntoken, borrower's collateral balance must be equal to or greater than ",(0,r.kt)("inlineCode",{parentName:"p"},"amount"),"\nand finally, the ",(0,r.kt)("inlineCode",{parentName:"p"},"asset_address")," must be accepted for borrowing in the\nsmart contract. After we calculate the liquidation price and borrow amount,\nwe ensure the contract has enough assets to provide for the borrower,\nand we also want the liquidation price of the collateral to be higher than\nthe borrowed amount. Since we are dealing with integers, entering a very\nlow amount (below 10) of collateral may result in the liquidation price being\nthe same as the borrowed amount, which could be exploited. We can surely\nhandle it in many different ways, but again, it is not the purpose of this\nexample so we will deal with it this way. When everything is alright, we will\ntransfer the collateral to the contract, mint an NFT, which stores the\ninformation about the loan, to the borrower, then transfer the asset to the\nborrower, and finally, mint the reserve token. We will mint the same amount\nthat we lent, and we will burn it after the loan is repaid or liquidated.\nThis reserve token will be used to track the amount of the asset which is\ncurrently borrowed."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[modifiers(when_not_paused)]\ndefault fn borrow_assets(\n    &mut self,\n    asset_address: AccountId,\n    collateral_address: AccountId,\n    amount: Balance,\n) -> Result<(), LendingError> {\n    // we will be using these often so we store them in variables\n    let borrower = Self::env().caller();\n    let contract = Self::env().account_id();\n    // ensure this asset is accepted as collateral\n    if !self.is_accepted_collateral(collateral_address) {\n        return Err(LendingError::AssetNotSupported)\n    }\n    // ensure the user gave allowance to the contract\n    if PSP22Ref::allowance(&collateral_address, borrower, contract) < amount {\n        return Err(LendingError::InsufficientAllowanceForCollateral)\n    }\n    // ensure the user has enough collateral assets\n    if PSP22Ref::balance_of(&collateral_address, borrower) < amount {\n        return Err(LendingError::InsufficientCollateralBalance)\n    }\n    let reserve_asset = get_reserve_asset(self, &asset_address)?;\n\n    // we will find out the price of deposited collateral\n    let price = get_asset_price(self, &amount, &collateral_address, &asset_address);\n    // we will set the liquidation price to be 75% of current price\n    let liquidation_price = (price * 75) / 100;\n    // borrow amount is 70% of collateral\n    let borrow_amount = (price * 70) / 100;\n    // ensure the liquidation price is greater than borrowed amount to avoid misuses\n    if borrow_amount >= liquidation_price {\n        return Err(LendingError::AmountNotSupported)\n    }\n    // ensure we have enough assets in the contract\n    if PSP22Ref::balance_of(&asset_address, contract) < borrow_amount {\n        return Err(LendingError::InsufficientBalanceInContract)\n    }\n    // we will transfer the collateral to the contract\n    PSP22Ref::transfer_from_builder(&collateral_address, borrower, contract, amount, Vec::<u8>::new())\n        .call_flags(ink::env::CallFlags::default().set_allow_reentry(true))\n        .fire()\n        .unwrap()?;\n    // create loan info\n    let loan_info = LoanInfo {\n        borrower,\n        collateral_token: collateral_address,\n        collateral_amount: amount,\n        borrow_token: asset_address,\n        borrow_amount,\n        liquidation_price,\n        timestamp: Self::env().block_timestamp(),\n        liquidated: false,\n    };\n\n    let load_account = self.data::<data::Data>().loan_account;\n    LoanRef::create_loan(&load_account, loan_info)?;\n    // transfer assets to borrower\n    PSP22Ref::transfer(&asset_address, borrower, borrow_amount, Vec::<u8>::new())?;\n    // mint `borrow_amount` of the reserve token\n    SharesRef::mint(&reserve_asset, contract, borrow_amount)?;\n    Ok(())\n}\n")))}u.isMDXComponent=!0}}]);