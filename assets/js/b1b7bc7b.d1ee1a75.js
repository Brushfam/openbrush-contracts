"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[5821],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var c=a.createContext({}),l=function(e){var t=a.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},p=function(e){var t=l(e.components);return a.createElement(c.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,c=e.parentName,p=i(e,["components","mdxType","originalType","parentName"]),u=l(n),d=r,f=u["".concat(c,".").concat(d)]||u[d]||m[d]||o;return n?a.createElement(f,s(s({ref:t},p),{},{components:n})):a.createElement(f,s({ref:t},p))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,s=new Array(o);s[0]=d;var i={};for(var c in t)hasOwnProperty.call(t,c)&&(i[c]=t[c]);i.originalType=e,i[u]="string"==typeof e?e:r,s[1]=i;for(var l=2;l<o;l++)s[l]=n[l];return a.createElement.apply(null,s)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},44684:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>l});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP22 Metadata"},s=void 0,i={unversionedId:"smart-contracts/PSP22/Extensions/metadata",id:"smart-contracts/PSP22/Extensions/metadata",title:"PSP22 Metadata",description:"This example shows how you can reuse the implementation of PSP22 token with the PSP22Metadata extension.",source:"@site/docs/smart-contracts/PSP22/Extensions/metadata.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/metadata",permalink:"/openbrush-contracts/next/smart-contracts/PSP22/Extensions/metadata",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/docs/smart-contracts/PSP22/Extensions/metadata.md",tags:[],version:"current",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP22 Metadata"},sidebar:"tutorialSidebar",previous:{title:"PSP22",permalink:"/openbrush-contracts/next/smart-contracts/PSP22/"},next:{title:"PSP22 Mintable",permalink:"/openbrush-contracts/next/smart-contracts/PSP22/Extensions/mintable"}},c={},l=[{value:"Step 1: Add imports and enable unstable feature",id:"step-1-add-imports-and-enable-unstable-feature",level:2},{value:"Step 2: Define storage",id:"step-2-define-storage",level:2},{value:"Step 3: Define constructor",id:"step-3-define-constructor",level:2},{value:"Final code",id:"final-code",level:2}],p={toc:l},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},p,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22"},"PSP22")," token with the ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/metadata.rs"},"PSP22Metadata")," extension."),(0,r.kt)("p",null,"First, you should implement basic version of ",(0,r.kt)("a",{parentName:"p",href:"/openbrush-contracts/next/smart-contracts/PSP22/"},"PSP22"),"."),(0,r.kt)("h2",{id:"step-1-add-imports-and-enable-unstable-feature"},"Step 1: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp22::extensions::metadata"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std, no_main)]\n\n#[openbrush::implementation(PSP22, PSP22Metadata)]\n#[openbrush::contract]\npub mod my_psp22 {\n    ...\n')),(0,r.kt)("h2",{id:"step-2-define-storage"},"Step 2: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to the metadata module data structure.\nThen you need to derive the ",(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," trait and mark the corresponding field with\nthe ",(0,r.kt)("inlineCode",{parentName:"p"},"#[storage_field]")," attribute. Deriving this trait allows you to reuse the\n",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Metadata")," extension in your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," implementation."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, Storage)]\npub struct Contract {\n    #[storage_field]\n    psp22: psp22::Data,\n    #[storage_field]\n    metadata: metadata::Data,\n}\n")),(0,r.kt)("h2",{id:"step-3-define-constructor"},"Step 3: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Metadata")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl Contract {\n    #[ink(constructor)]\n    pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {\n        let mut instance = Self::default();\n        let caller = instance.env().caller();\n\n        instance.metadata.name.set(&name);\n        instance.metadata.symbol.set(&symbol);\n        instance.metadata.decimals.set(&decimal);\n\n        psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");\n\n        instance\n    }\n}\n')),(0,r.kt)("h2",{id:"final-code"},"Final code"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std, no_main)]\n\n#[openbrush::implementation(PSP22, PSP22Metadata)]\n#[openbrush::contract]\npub mod my_psp22 {\n    use openbrush::traits::Storage;\n\n    #[ink(storage)]\n    #[derive(Default, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        psp22: psp22::Data,\n        #[storage_field]\n        metadata: metadata::Data,\n    }\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {\n            let mut instance = Self::default();\n            let caller = instance.env().caller();\n\n            instance.metadata.name.set(&name);\n            instance.metadata.symbol.set(&symbol);\n            instance.metadata.decimals.set(&decimal);\n\n            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");\n\n            instance\n        }\n    }\n}\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Brushfam/openbrush-contracts/tree/main/examples/psp22_extensions/metadata"},"PSP22 Metadata"),"."))}m.isMDXComponent=!0}}]);