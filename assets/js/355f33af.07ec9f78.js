"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[7703],{3905:(e,t,n)=>{n.d(t,{Zo:()=>l,kt:()=>P});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),c=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},l=function(e){var t=c(e.components);return a.createElement(p.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,p=e.parentName,l=s(e,["components","mdxType","originalType","parentName"]),u=c(n),m=r,P=u["".concat(p,".").concat(m)]||u[m]||d[m]||o;return n?a.createElement(P,i(i({ref:t},l),{},{components:n})):a.createElement(P,i({ref:t},l))}));function P(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=m;var s={};for(var p in t)hasOwnProperty.call(t,p)&&(s[p]=t[p]);s.originalType=e,s[u]="string"==typeof e?e:r,i[1]=s;for(var c=2;c<o;c++)i[c]=n[c];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},21421:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>c});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP35 Metadata"},i=void 0,s={unversionedId:"smart-contracts/PSP35/Extensions/metadata",id:"version-2.1.0/smart-contracts/PSP35/Extensions/metadata",title:"PSP35 Metadata",description:"This example shows how you can reuse the implementation of PSP35 token with PSP35Metadata extension.",source:"@site/versioned_docs/version-2.1.0/smart-contracts/PSP35/Extensions/metadata.md",sourceDirName:"smart-contracts/PSP35/Extensions",slug:"/smart-contracts/PSP35/Extensions/metadata",permalink:"/openbrush-contracts/2.1.0/smart-contracts/PSP35/Extensions/metadata",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-2.1.0/smart-contracts/PSP35/Extensions/metadata.md",tags:[],version:"2.1.0",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP35 Metadata"},sidebar:"tutorialSidebar",previous:{title:"PSP35 Enumerable",permalink:"/openbrush-contracts/2.1.0/smart-contracts/PSP35/Extensions/enumerable"},next:{title:"PSP35 Batch",permalink:"/openbrush-contracts/2.1.0/smart-contracts/PSP35/Extensions/batch"}},p={},c=[{value:"Step 1: Add imports and enable unstable feature",id:"step-1-add-imports-and-enable-unstable-feature",level:2},{value:"Step 2: Define storage",id:"step-2-define-storage",level:2},{value:"Step 3: Inherit logic",id:"step-3-inherit-logic",level:2},{value:"Step 4: Define constructor",id:"step-4-define-constructor",level:2}],l={toc:c},u="wrapper";function d(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35"},"PSP35")," token with ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp35/extensions/metadata.rs"},"PSP35Metadata")," extension."),(0,r.kt)("h2",{id:"step-1-add-imports-and-enable-unstable-feature"},"Step 1: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"openbrush::contracts::psp35::extensions::metadata"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp35 {\n    use openbrush::contracts::psp35::extensions::metadata::*;\n    use ink_prelude::string::String;\n    use ink_storage::traits::SpreadAllocate;\n...\n')),(0,r.kt)("h2",{id:"step-2-define-storage"},"Step 2: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35MetadataStorage")," trait in addition to your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35Storage")," field. Then you need to derive the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35MetadataStorage")," trait and mark the corresponding field with the ",(0,r.kt)("inlineCode",{parentName:"p"},"#[PSP35MetadataStorageField]")," attribute. Deriving this trait allows you to reuse the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35Metadata")," extension in your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35")," implementation."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[derive(Default, SpreadAllocate, PSP35Storage, PSP35MetadataStorage)]\n#[ink(storage)]\npub struct MyPSP35 {\n    #[PSP35StorageField]\n    psp35: PSP35Data,\n    #[PSP35MetadataStorageField]\n    metadata: PSP35MetadataData,\n}\n")),(0,r.kt)("h2",{id:"step-3-inherit-logic"},"Step 3: Inherit logic"),(0,r.kt)("p",null,"Inherit implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35Metadata")," trait. You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"\nimpl PSP35 for MyPSP35 {}\n\nimpl PSP35Metadata for MyPSP35 {}\n")),(0,r.kt)("h2",{id:"step-4-define-constructor"},"Step 4: Define constructor"),(0,r.kt)("p",null,"Define constructor. Your ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP35Metadata")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyPSP35 {\n    #[ink(constructor)]\n    pub fn new(id: Id, key: Vec<u8>, attribute: Vec<u8>) -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n            instance.metadata.attributes.insert(&(id, key), &attribute);\n        })\n    }\n}\n")),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp35_extensions/metadata"},"PSP35 Metadata"),"."),(0,r.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,r.kt)("a",{parentName:"p",href:"/openbrush-contracts/2.1.0/smart-contracts/PSP35/"},"PSP35"),"."))}d.isMDXComponent=!0}}]);