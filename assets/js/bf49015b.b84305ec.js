"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[61027],{3905:(e,t,n)=>{n.d(t,{Zo:()=>l,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),c=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},l=function(e){var t=c(e.components);return a.createElement(p.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,p=e.parentName,l=i(e,["components","mdxType","originalType","parentName"]),u=c(n),d=r,f=u["".concat(p,".").concat(d)]||u[d]||m[d]||o;return n?a.createElement(f,s(s({ref:t},l),{},{components:n})):a.createElement(f,s({ref:t},l))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,s=new Array(o);s[0]=d;var i={};for(var p in t)hasOwnProperty.call(t,p)&&(i[p]=t[p]);i.originalType=e,i[u]="string"==typeof e?e:r,s[1]=i;for(var c=2;c<o;c++)s[c]=n[c];return a.createElement.apply(null,s)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},74928:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>c});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP22 Metadata"},s=void 0,i={unversionedId:"smart-contracts/PSP22/Extensions/metadata",id:"version-3.0.0-beta/smart-contracts/PSP22/Extensions/metadata",title:"PSP22 Metadata",description:"This example shows how you can reuse the implementation of PSP22 token with the PSP22Metadata extension.",source:"@site/versioned_docs/version-3.0.0-beta/smart-contracts/PSP22/Extensions/metadata.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/metadata",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/Extensions/metadata",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-3.0.0-beta/smart-contracts/PSP22/Extensions/metadata.md",tags:[],version:"3.0.0-beta",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP22 Metadata"},sidebar:"tutorialSidebar",previous:{title:"PSP22",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/"},next:{title:"PSP22 Mintable",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/Extensions/mintable"}},p={},c=[{value:"Step 1: Add imports and enable unstable feature",id:"step-1-add-imports-and-enable-unstable-feature",level:2},{value:"Step 2: Define storage",id:"step-2-define-storage",level:2},{value:"Step 3: Inherit logic",id:"step-3-inherit-logic",level:2},{value:"Step 4: Define constructor",id:"step-4-define-constructor",level:2},{value:"Final code",id:"final-code",level:2}],l={toc:c},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22"},"PSP22")," token with the ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22/extensions/metadata.rs"},"PSP22Metadata")," extension."),(0,r.kt)("p",null,"First, you should implement basic version of ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22"),"."),(0,r.kt)("h2",{id:"step-1-add-imports-and-enable-unstable-feature"},"Step 1: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp22::extensions::metadata"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp22 {\n    use openbrush::contracts::psp22::extensions::metadata::*;\n')),(0,r.kt)("h2",{id:"step-2-define-storage"},"Step 2: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to the metadata module data structure.\nThen you need to derive the ",(0,r.kt)("inlineCode",{parentName:"p"},"Storage")," trait and mark the corresponding field with\nthe ",(0,r.kt)("inlineCode",{parentName:"p"},"#[storage_field]")," attribute. Deriving this trait allows you to reuse the\n",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Metadata")," extension in your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," implementation."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, Storage)]\npub struct Contract {\n    ...\n    #[storage_field]\n    metadata: metadata::Data,\n}\n")),(0,r.kt)("h2",{id:"step-3-inherit-logic"},"Step 3: Inherit logic"),(0,r.kt)("p",null,"Inherit the implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Metadata")," trait. You can customize (override)\nmethods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("p",null,"Inherit the implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," trait."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl PSP22 for Contract {}\n\nimpl PSP22Metadata for Contract {}\n")),(0,r.kt)("h2",{id:"step-4-define-constructor"},"Step 4: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22Metadata")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'impl Contract {\n    #[ink(constructor)]\n    pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {\n        let mut instance = Self::default();\n\n        instance.metadata.name = name;\n        instance.metadata.symbol = symbol;\n        instance.metadata.decimals = decimal;\n        instance\n            ._mint_to(Self::env().caller(), total_supply)\n            .expect("Should mint total_supply");\n        \n        instance\n    }\n}\n')),(0,r.kt)("h2",{id:"final-code"},"Final code"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp22 {\n    use openbrush::traits::String;\n    use openbrush::{\n        contracts::psp22::extensions::metadata::*,\n        traits::Storage,\n    };\n\n    #[ink(storage)]\n    #[derive(Default, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        psp22: psp22::Data,\n        #[storage_field]\n        metadata: metadata::Data,\n    }\n\n    impl PSP22 for Contract {}\n\n    impl PSP22Metadata for Contract {}\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {\n            let mut instance = Self::default();\n\n            instance.metadata.name = name;\n            instance.metadata.symbol = symbol;\n            instance.metadata.decimals = decimal;\n            instance\n                ._mint_to(Self::env().caller(), total_supply)\n                .expect("Should mint total_supply");\n            \n            instance\n        }\n    }\n}\n')),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp22_extensions/metadata"},"PSP22 Metadata"),"."),(0,r.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22"),"."))}m.isMDXComponent=!0}}]);