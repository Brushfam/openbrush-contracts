"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[16440],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var s=r.createContext({}),p=function(e){var t=r.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},c=function(e){var t=p(e.components);return r.createElement(s.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},d=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,c=l(e,["components","mdxType","originalType","parentName"]),u=p(n),d=a,f=u["".concat(s,".").concat(d)]||u[d]||m[d]||o;return n?r.createElement(f,i(i({ref:t},c),{},{components:n})):r.createElement(f,i({ref:t},c))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[u]="string"==typeof e?e:a,i[1]=l;for(var p=2;p<o;p++)i[p]=n[p];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}d.displayName="MDXCreateElement"},31590:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>m,frontMatter:()=>o,metadata:()=>l,toc:()=>p});var r=n(87462),a=(n(67294),n(3905));const o={sidebar_position:7,title:"Payment Splitter"},i=void 0,l={unversionedId:"smart-contracts/payment-splitter",id:"version-v2.3.0/smart-contracts/payment-splitter",title:"Payment Splitter",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-v2.3.0/smart-contracts/payment-splitter.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/payment-splitter",permalink:"/openbrush-contracts/v2.3.0/smart-contracts/payment-splitter",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-v2.3.0/smart-contracts/payment-splitter.md",tags:[],version:"v2.3.0",sidebarPosition:7,frontMatter:{sidebar_position:7,title:"Payment Splitter"},sidebar:"tutorialSidebar",previous:{title:"Pausable",permalink:"/openbrush-contracts/v2.3.0/smart-contracts/pausable"},next:{title:"PSP22",permalink:"/openbrush-contracts/v2.3.0/smart-contracts/PSP22/"}},s={},p=[{value:"Step 1: Import default implementation",id:"step-1-import-default-implementation",level:2},{value:"Step 2: Define constructor",id:"step-2-define-constructor",level:2},{value:"Step 3 (Optional): Customize your contract",id:"step-3-optional-customize-your-contract",level:2}],c={toc:p},u="wrapper";function m(e){let{components:t,...n}=e;return(0,a.kt)(u,(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/src/finance/payment_splitter"},"payment-splitter"),"."),(0,a.kt)("h2",{id:"step-1-import-default-implementation"},"Step 1: Import default implementation"),(0,a.kt)("p",null,"With ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,a.kt)("inlineCode",{parentName:"a"},"Cargo.toml")),",\nyou need to import the ",(0,a.kt)("inlineCode",{parentName:"p"},"payment_splitter")," module, enable the corresponding feature, and embed the module data structure\nas described in ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush"},"that section"),"."),(0,a.kt)("p",null,"The main trait is ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitter"),"."),(0,a.kt)("h2",{id:"step-2-define-constructor"},"Step 2: Define constructor"),(0,a.kt)("p",null,"Define constructor where you init payees and shares."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'impl Contract {\n   #[ink(constructor)]\n   pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {\n      ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n         instance._init(payees_and_shares).expect("Should init");\n      })\n   }\n}\n')),(0,a.kt)("p",null,"You can check an example of the usage of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/payment_splitter"},"PaymentSplitter"),"."),(0,a.kt)("h2",{id:"step-3-optional-customize-your-contract"},"Step 3 (Optional): Customize your contract"),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitter")," trait defines and has default implementations for the core payment splitter functions.\nAdditional functionality with ",(0,a.kt)("em",{parentName:"p"},"some")," predefined functions is available through the ",(0,a.kt)("inlineCode",{parentName:"p"},"payment_splitter::Internal")," trait.\nLikely the most common function to use from this internal trait will be ",(0,a.kt)("inlineCode",{parentName:"p"},"_release_all"),". This allows you to payout all\n",(0,a.kt)("inlineCode",{parentName:"p"},"payees")," stored in the contract at once. To add this function to your contract, simply define a new publicly dispatchable\nfunction (i.e. ",(0,a.kt)("inlineCode",{parentName:"p"},"#[ink(message)]"),") called ",(0,a.kt)("inlineCode",{parentName:"p"},"release_all")," and have it call the internal ",(0,a.kt)("inlineCode",{parentName:"p"},"_release_all")," function using ",(0,a.kt)("inlineCode",{parentName:"p"},"self"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_payment_splitter {\n    use ink_prelude::vec::Vec;\n    use ink_storage::traits::SpreadAllocate;\n    use openbrush::contracts::payment_splitter::*;\n    use openbrush::traits::Storage;\n\n    #[ink(storage)]\n    #[derive(Default, SpreadAllocate, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        splitter: payment_splitter::Data,\n    }\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {\n            ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n                instance._init(payees_and_shares).expect("Should init");\n            })\n        }\n\n        /// Payout all payees at once.\n        /// Delete this method if you don\'t want this functionality in your version of the payment splitter.\n        #[ink(message)]\n        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {\n            // `_release_all()` is an internal method defined by the `payment_splitter::Internal` trait\n            self._release_all()\n        }\n    }\n\n    impl PaymentSplitter for Contract {}\n}\n')),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"_add_payee")," function is also available in the ",(0,a.kt)("inlineCode",{parentName:"p"},"payment_splitter::Internal")," trait and can be added to\nyour contract in the same way as ",(0,a.kt)("inlineCode",{parentName:"p"},"_release_all"),"."))}m.isMDXComponent=!0}}]);