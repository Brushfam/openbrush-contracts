"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[90668],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function o(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},i=Object.keys(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)n=i[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var p=r.createContext({}),l=function(e){var t=r.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},c=function(e){var t=l(e.components);return r.createElement(p.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,i=e.originalType,p=e.parentName,c=o(e,["components","mdxType","originalType","parentName"]),u=l(n),m=a,f=u["".concat(p,".").concat(m)]||u[m]||d[m]||i;return n?r.createElement(f,s(s({ref:t},c),{},{components:n})):r.createElement(f,s({ref:t},c))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var i=n.length,s=new Array(i);s[0]=m;var o={};for(var p in t)hasOwnProperty.call(t,p)&&(o[p]=t[p]);o.originalType=e,o[u]="string"==typeof e?e:a,s[1]=o;for(var l=2;l<i;l++)s[l]=n[l];return r.createElement.apply(null,s)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},45181:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>s,default:()=>d,frontMatter:()=>i,metadata:()=>o,toc:()=>l});var r=n(87462),a=(n(67294),n(3905));const i={sidebar_position:7,title:"Payment Splitter"},s=void 0,o={unversionedId:"smart-contracts/payment-splitter",id:"version-1.1.0/smart-contracts/payment-splitter",title:"Payment Splitter",description:"This example shows how you can reuse the implementation of",source:"@site/versioned_docs/version-1.1.0/smart-contracts/payment-splitter.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/payment-splitter",permalink:"/openbrush-contracts/1.1.0/smart-contracts/payment-splitter",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.1.0/smart-contracts/payment-splitter.md",tags:[],version:"1.1.0",sidebarPosition:7,frontMatter:{sidebar_position:7,title:"Payment Splitter"},sidebar:"tutorialSidebar",previous:{title:"Pausable",permalink:"/openbrush-contracts/1.1.0/smart-contracts/pausable"},next:{title:"Timelock Controller",permalink:"/openbrush-contracts/1.1.0/smart-contracts/timelock-controller"}},p={},l=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports",id:"step-2-add-imports",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2}],c={toc:l},u="wrapper";function d(e){let{components:t,...n}=e;return(0,a.kt)(u,(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of\n",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/finance/payment-splitter"},"payment-splitter"),"."),(0,a.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,a.kt)("p",null,"Include dependencies to ",(0,a.kt)("inlineCode",{parentName:"p"},"payment-splitter")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"brush")," in the cargo file."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-toml"},'[dependencies]\nink_primitives = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_metadata = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false, features = ["derive"], optional = true }\nink_env = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_storage = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_lang = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\nink_prelude = { tag = "v3.0.0-rc6", git = "https://github.com/paritytech/ink", default-features = false }\n\nscale = { package = "parity-scale-codec", version = "2", default-features = false, features = ["derive"] }\nscale-info = { version = "1", default-features = false, features = ["derive"], optional = true }\n\n# These dependencies\npayment-splitter = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }\nbrush = { tag = "v1.0.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false }\n\n\n# payment-splitter uses dividing inside, so your version of rust can require you to disable check overflow.\n[profile.dev]\noverflow-checks = false\n\n[profile.release]\noverflow-checks = false\n\n[features]\ndefault = ["std"]\nstd = [\n   "ink_primitives/std",\n   "ink_metadata",\n   "ink_metadata/std",\n   "ink_env/std",\n   "ink_storage/std",\n   "ink_lang/std",\n   "scale/std",\n   "scale-info",\n   "scale-info/std",\n\n   # These dependencies   \n   "payment-splitter/std",\n   "brush/std",\n]\n')),(0,a.kt)("h2",{id:"step-2-add-imports"},"Step 2: Add imports"),(0,a.kt)("p",null,"Replace ",(0,a.kt)("inlineCode",{parentName:"p"},"ink::contract")," macro by ",(0,a.kt)("inlineCode",{parentName:"p"},"brush::contract"),".\nImport ",(0,a.kt)("strong",{parentName:"p"},"everything")," from ",(0,a.kt)("inlineCode",{parentName:"p"},"payment_splitter::traits"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[brush::contract]\npub mod my_payment_splitter {\n   use payment_splitter::traits::*;\n   use ink_prelude::vec::Vec;\n")),(0,a.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,a.kt)("p",null,"Declare storage struct and declare the field related to ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitterStorage"),"\nThen you need to derive ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitterStorage")," trait and mark corresponding field\nwith ",(0,a.kt)("inlineCode",{parentName:"p"},"#[PaymentSplitterStorageField]")," attribute. Deriving this trait allows you to reuse\nthe default implementation of ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitter"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PaymentSplitterStorage)]\npub struct SplitterStruct {\n   #[PaymentSplitterStorageField]\n   splitter: PaymentSplitterData,\n}\n")),(0,a.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,a.kt)("p",null,"Inherit the implementation of ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitter"),". You can customize (override) methods in this ",(0,a.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"impl PaymentSplitter for SplitterStruct {}\n")),(0,a.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,a.kt)("p",null,"Define constructor. Your basic version of ",(0,a.kt)("inlineCode",{parentName:"p"},"PaymentSplitter")," contract is ready!"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"impl SplitterStruct {\n   #[ink(constructor)]\n   pub fn new(payees: Vec<AccountId>, shares: Vec<Balance>) -> Self {\n      let mut instance = Self::default();\n      instance._init(payees, shares);\n      instance\n   }\n}\n")))}d.isMDXComponent=!0}}]);