"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[80256],{3905:(e,n,t)=>{t.d(n,{Zo:()=>d,kt:()=>f});var a=t(67294);function r(e,n,t){return n in e?Object.defineProperty(e,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):e[n]=t,e}function o(e,n){var t=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);n&&(a=a.filter((function(n){return Object.getOwnPropertyDescriptor(e,n).enumerable}))),t.push.apply(t,a)}return t}function s(e){for(var n=1;n<arguments.length;n++){var t=null!=arguments[n]?arguments[n]:{};n%2?o(Object(t),!0).forEach((function(n){r(e,n,t[n])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(t)):o(Object(t)).forEach((function(n){Object.defineProperty(e,n,Object.getOwnPropertyDescriptor(t,n))}))}return e}function i(e,n){if(null==e)return{};var t,a,r=function(e,n){if(null==e)return{};var t,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)t=o[a],n.indexOf(t)>=0||(r[t]=e[t]);return r}(e,n);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)t=o[a],n.indexOf(t)>=0||Object.prototype.propertyIsEnumerable.call(e,t)&&(r[t]=e[t])}return r}var l=a.createContext({}),c=function(e){var n=a.useContext(l),t=n;return e&&(t="function"==typeof e?e(n):s(s({},n),e)),t},d=function(e){var n=c(e.components);return a.createElement(l.Provider,{value:n},e.children)},p="mdxType",m={inlineCode:"code",wrapper:function(e){var n=e.children;return a.createElement(a.Fragment,{},n)}},u=a.forwardRef((function(e,n){var t=e.components,r=e.mdxType,o=e.originalType,l=e.parentName,d=i(e,["components","mdxType","originalType","parentName"]),p=c(t),u=r,f=p["".concat(l,".").concat(u)]||p[u]||m[u]||o;return t?a.createElement(f,s(s({ref:n},d),{},{components:t})):a.createElement(f,s({ref:n},d))}));function f(e,n){var t=arguments,r=n&&n.mdxType;if("string"==typeof e||r){var o=t.length,s=new Array(o);s[0]=u;var i={};for(var l in n)hasOwnProperty.call(n,l)&&(i[l]=n[l]);i.originalType=e,i[p]="string"==typeof e?e:r,s[1]=i;for(var c=2;c<o;c++)s[c]=t[c];return a.createElement.apply(null,s)}return a.createElement.apply(null,t)}u.displayName="MDXCreateElement"},97952:(e,n,t)=>{t.r(n),t.d(n,{assets:()=>l,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>c});var a=t(87462),r=(t(67294),t(3905));const o={sidebar_position:9,title:"Lending contract"},s=void 0,i={unversionedId:"smart-contracts/example/contract",id:"version-v4.0.0-beta/smart-contracts/example/contract",title:"Lending contract",description:"The main logic of the LendingContract is defined in the impls/lending directory.",source:"@site/versioned_docs/version-v4.0.0-beta/smart-contracts/example/contract.md",sourceDirName:"smart-contracts/example",slug:"/smart-contracts/example/contract",permalink:"/openbrush-contracts/smart-contracts/example/contract",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-v4.0.0-beta/smart-contracts/example/contract.md",tags:[],version:"v4.0.0-beta",sidebarPosition:9,frontMatter:{sidebar_position:9,title:"Lending contract"},sidebar:"tutorialSidebar",previous:{title:"Errors",permalink:"/openbrush-contracts/smart-contracts/example/errors"},next:{title:"Notes about methods",permalink:"/openbrush-contracts/smart-contracts/example/implementation"}},l={},c=[{value:"Add dependencies",id:"add-dependencies",level:2},{value:"Define the contract storage",id:"define-the-contract-storage",level:2},{value:"Implement traits",id:"implement-traits",level:2},{value:"Define the constructor",id:"define-the-constructor",level:2}],d={toc:c},p="wrapper";function m(e){let{components:n,...t}=e;return(0,r.kt)(p,(0,a.Z)({},d,t,{components:n,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"The main logic of the ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract")," is defined in the ",(0,r.kt)("inlineCode",{parentName:"p"},"impls/lending"),' directory.\nIn this file, we only need to "inherit" it.'),(0,r.kt)("h2",{id:"add-dependencies"},"Add dependencies"),(0,r.kt)("p",null,(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract")," instantiates the ",(0,r.kt)("inlineCode",{parentName:"p"},"SharesContract")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"LoanContract"),", so we\nshould import them as ",(0,r.kt)("inlineCode",{parentName:"p"},"ink-as-dependency"),". Also we want to use the ",(0,r.kt)("inlineCode",{parentName:"p"},"AccessControl"),"\nand ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),' from OpenBrush, so we import them too. We also want to "inherit" the\nimplementation of ',(0,r.kt)("inlineCode",{parentName:"p"},"Lending")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingPermissioned")," traits defined in the ",(0,r.kt)("inlineCode",{parentName:"p"},"lending_project")," crate."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'[package]\nname = "lending_contract"\nversion= "4.0.0-beta"\nauthors = ["Brushfam <dominik.krizo@727.ventures>"]\nedition = "2021"\n\n[dependencies]\nink = { version = "4.2.1", default-features = false }\nscale = { package = "parity-scale-codec", version = "3", default-features = false, features = ["derive"] }\nscale-info = { version = "2.6", default-features = false, features = ["derive"], optional = true }\n\n# These dependencies\nshares_contract = { path = "../shares", default-features = false, features = ["ink-as-dependency"]  }\nloan_contract = { path = "../loan", default-features = false, features = ["ink-as-dependency"]  }\nlending_project = { path = "../..", default-features = false }\nopenbrush = { git = "https://github.com/Brushfam/openbrush-contracts", branch = "develop", default-features = false, features = ["pausable", "access_control"] }\n\n[lib]\nname = "lending_contract"\npath = "lib.rs"\n\n\n[features]\ndefault = ["std"]\nstd = [\n    "ink/std",\n    "scale/std",\n    "scale-info",\n    "scale-info/std",\n\n    # These dependencies\n    "loan_contract/std",\n    "shares_contract/std",\n    "openbrush/std",\n]\nink-as-dependency = []\n\n[profile.dev]\ncodegen-units = 16\n\n[profile.release]\noverflow-checks = false\n')),(0,r.kt)("h2",{id:"define-the-contract-storage"},"Define the contract storage"),(0,r.kt)("p",null,"As described earlier, we want our smart contract to be paused by the Manager account.\nTo do that, we need our contract to be ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable")," and we need a manager role.\nWe can do this with the ",(0,r.kt)("inlineCode",{parentName:"p"},"AccessControl"),". Also, we want to use the data from lending that we have declared.\nSo we will declare a struct and derive all the needed traits."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, Storage)]\npub struct LendingContract {\n    #[storage_field]\n    access: access_control::Data,\n    #[storage_field]\n    pause: pausable::Data,\n    #[storage_field]\n    lending: lending::data::Data,\n}\n")),(0,r.kt)("h2",{id:"implement-traits"},"Implement traits"),(0,r.kt)("p",null,'We need to "inherit" the implementation of ',(0,r.kt)("inlineCode",{parentName:"p"},"AccessControll"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"Lending"),",\n",(0,r.kt)("inlineCode",{parentName:"p"},"LendingPermissioned")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"lending::Internal"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl lending::Internal for LendingContract {}\n\nimpl LendingImpl for LendingContract {}\n\nimpl Lending for LendingContract {\n    #[ink(message)]\n    fn total_asset(&self, asset_address: AccountId) -> Result<Balance, LendingError> {\n        LendingImpl::total_asset(self, asset_address)\n    }\n    // other methods should be implemented here as the one above\n}\n\nimpl LendingPermissionedImpl for LendingContract {}\n\nimpl LendingPermissioned for LendingContract {\n    #[ink(message)]\n    fn deposit(&mut self, asset_address: AccountId, amount: Balance) -> Result<(), LendingError> {\n        LendingPermissionedImpl::deposit(self, asset_address, amount)\n    }\n    // other methods should be implemented here as the one above\n}\n\nimpl lending::Instantiator for LendingContract {\n    fn _instantiate_shares_contract(&self, contract_name: &str, contract_symbol: &str) -> AccountId {\n        let code_hash = self.lending.shares_contract_code_hash;\n\n        let salt = (<Self as DefaultEnv>::env().block_timestamp(), contract_name).encode();\n\n        let hash = xxh32(&salt, 0).to_le_bytes();\n\n        let contract =\n            SharesContractRef::new(Some(String::from(contract_name)), Some(String::from(contract_symbol)))\n                .endowment(0)\n                .code_hash(code_hash)\n                .salt_bytes(&hash[..4])\n                .instantiate();\n        contract.to_account_id()\n    }\n}\n")),(0,r.kt)("p",null,"Now the ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract")," has functionality of all that traits."),(0,r.kt)("h2",{id:"define-the-constructor"},"Define the constructor"),(0,r.kt)("p",null,"Finally, we will add a constructor, in which we will initiate the admin of\nthe contract, to whom we will also grant the manager role declared before,\nand we will also instantiate the ",(0,r.kt)("inlineCode",{parentName:"p"},"LoanContract")," here and store its AccountId\nin ",(0,r.kt)("inlineCode",{parentName:"p"},"LendingContract"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl LendingContract {\n    /// constructor with name and symbol\n    #[ink(constructor, payable)]\n    pub fn new(shares_hash: Hash, loan_hash: Hash) -> Self {\n        \n        let mut instance = Self::default();\n\n        let caller = Self::env().caller();\n        instance._init_with_admin(caller);\n        instance.grant_role(MANAGER, caller).expect("Can not set manager role");\n        instance.lending.shares_contract_code_hash = shares_hash;\n        // instantiate NFT contract and store its account id\n        let nft = LoanContractRef::new()\n            .endowment(0)\n            .code_hash(loan_hash)\n            .salt_bytes(&[0xDE, 0xAD, 0xBE, 0xEF])\n            .instantiate()\n            .unwrap();\n        instance.lending.loan_account = nft.to_account_id();\n       \n        instance\n    }\n}\n')))}m.isMDXComponent=!0}}]);