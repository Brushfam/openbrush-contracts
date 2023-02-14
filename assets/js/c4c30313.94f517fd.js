"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[72048],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var r=n(67294);function a(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,r)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){a(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,r,a=function(e,t){if(null==e)return{};var n,r,a={},o=Object.keys(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||(a[n]=e[n]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(r=0;r<o.length;r++)n=o[r],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(a[n]=e[n])}return a}var l=r.createContext({}),p=function(e){var t=r.useContext(l),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},c=function(e){var t=p(e.components);return r.createElement(l.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},m=r.forwardRef((function(e,t){var n=e.components,a=e.mdxType,o=e.originalType,l=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),u=p(n),m=a,f=u["".concat(l,".").concat(m)]||u[m]||d[m]||o;return n?r.createElement(f,i(i({ref:t},c),{},{components:n})):r.createElement(f,i({ref:t},c))}));function f(e,t){var n=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=n.length,i=new Array(o);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[u]="string"==typeof e?e:a,i[1]=s;for(var p=2;p<o;p++)i[p]=n[p];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}m.displayName="MDXCreateElement"},70244:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>p});var r=n(87462),a=(n(67294),n(3905));const o={sidebar_position:1,title:"PSP721"},i=void 0,s={unversionedId:"smart-contracts/PSP721/psp721",id:"version-1.2.0/smart-contracts/PSP721/psp721",title:"PSP721",description:"This example shows how you can reuse the implementation of PSP721 token. Also, this example shows how you can customize the logic, for example, to track the number of tokens minted with next_id, increasing it with each new token minted, securing a unique id for each token.",source:"@site/versioned_docs/version-1.2.0/smart-contracts/PSP721/psp721.md",sourceDirName:"smart-contracts/PSP721",slug:"/smart-contracts/PSP721/",permalink:"/openbrush-contracts/1.2.0/smart-contracts/PSP721/",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.2.0/smart-contracts/PSP721/psp721.md",tags:[],version:"1.2.0",sidebarPosition:1,frontMatter:{sidebar_position:1,title:"PSP721"},sidebar:"tutorialSidebar",previous:{title:"PSP22 Token Timelock",permalink:"/openbrush-contracts/1.2.0/smart-contracts/PSP22/Utils/token-timelock"},next:{title:"PSP721 Metadata",permalink:"/openbrush-contracts/1.2.0/smart-contracts/PSP721/Extensions/metadata"}},l={},p=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2},{value:"Step 6: Customize your contract",id:"step-6-customize-your-contract",level:2}],c={toc:p},u="wrapper";function d(e){let{components:t,...n}=e;return(0,a.kt)(u,(0,r.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"This example shows how you can reuse the implementation of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/token/psp721"},"PSP721")," token. Also, this example shows how you can customize the logic, for example, to track the number of tokens minted with ",(0,a.kt)("inlineCode",{parentName:"p"},"next_id"),", increasing it with each new token minted, securing a unique id for each token."),(0,a.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,a.kt)("p",null,"Include ",(0,a.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,a.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,a.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of PSP721 via ",(0,a.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.2.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["psp721"] }\n')),(0,a.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,a.kt)("p",null,"Use ",(0,a.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,a.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,a.kt)("strong",{parentName:"p"},"everything")," from ",(0,a.kt)("inlineCode",{parentName:"p"},"brush::contracts::psp721"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_psp721 {\n    use brush::contracts::psp721::*;\n')),(0,a.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,a.kt)("p",null,"Declare storage struct and declare the field related to the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP721Storage")," trait. Then you need to derive the ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP721Storage")," trait and mark the corresponding field with the ",(0,a.kt)("inlineCode",{parentName:"p"},"#[PSP721StorageField]")," attribute. Deriving this trait allows you to reuse the default implementation of ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP721"),"."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PSP721Storage)]\npub struct MyPSP721 {\n    #[PSP721StorageField]\n    psp721: PSP721Data,\n}\n")),(0,a.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,a.kt)("p",null,"Inherit implementations of ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP721")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP721Metadata")," traits. You can customize (override) methods in this ",(0,a.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"impl PSP721 for MyPSP721 {}\n\n")),(0,a.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,a.kt)("p",null,"Define constructor. Your basic version of ",(0,a.kt)("inlineCode",{parentName:"p"},"PSP721")," contract is ready!"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"impl MyPSP721 {\n    #[ink(constructor)]\n    pub fn new() -> Self {\n        Self::default()\n    }\n}\n")),(0,a.kt)("h2",{id:"step-6-customize-your-contract"},"Step 6: Customize your contract"),(0,a.kt)("p",null,"Customize it by adding logic for tracking the number of minted tokens. It will contain a custom ",(0,a.kt)("inlineCode",{parentName:"p"},"mint_token")," function which will handle the id of the newly minted token. Also, we will add the ",(0,a.kt)("inlineCode",{parentName:"p"},"next_id: u8")," field to the structure, which will be increased with each newly minted token. This way we will make sure there will always be added a token with a unique id. "),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, PSP721Storage)]\npub struct MyPSP721 {\n    #[PSP721StorageField]\n    psp721: PSP721Data,\n    next_id: u8,\n}\n\nimpl PSP721 for MyPSP721 {}\n\nimpl MyPSP721 {\n    #[ink(constructor)]\n    pub fn new() -> Self {\n        Self::default()\n    }\n\n    #[ink(message)]\n    pub fn mint_token(&mut self) -> Result<(), PSP721Error> {\n        self._mint([self.next_id; 32])?;\n        self.next_id += 1;\n        Ok(())\n    }\n}\n")),(0,a.kt)("p",null,"You can check an example of the usage of ",(0,a.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/psp721"},"PSP721"),".\nAlso you can use extensions for psp721 token:"),(0,a.kt)("p",null,(0,a.kt)("a",{parentName:"p",href:"Extensions/metadata"},"PSP721Metadata"),": metadata for PSP721."),(0,a.kt)("p",null,(0,a.kt)("a",{parentName:"p",href:"Extensions/mintable"},"PSP721Mintable"),": creation of new tokens."),(0,a.kt)("p",null,(0,a.kt)("a",{parentName:"p",href:"Extensions/burnable"},"PSP721Burnable"),": destruction of contract's tokens."))}d.isMDXComponent=!0}}]);