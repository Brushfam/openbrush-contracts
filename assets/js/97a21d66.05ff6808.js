"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[58447],{3905:(e,t,n)=>{n.d(t,{Zo:()=>s,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function c(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var d=a.createContext({}),l=function(e){var t=a.useContext(d),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},s=function(e){var t=l(e.components);return a.createElement(d.Provider,{value:t},e.children)},u="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,d=e.parentName,s=c(e,["components","mdxType","originalType","parentName"]),u=l(n),m=r,f=u["".concat(d,".").concat(m)]||u[m]||p[m]||o;return n?a.createElement(f,i(i({ref:t},s),{},{components:n})):a.createElement(f,i({ref:t},s))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=m;var c={};for(var d in t)hasOwnProperty.call(t,d)&&(c[d]=t[d]);c.originalType=e,c[u]="string"==typeof e?e:r,i[1]=c;for(var l=2;l<o;l++)i[l]=n[l];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}m.displayName="MDXCreateElement"},57881:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>d,contentTitle:()=>i,default:()=>u,frontMatter:()=>o,metadata:()=>c,toc:()=>l});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:4,title:"Diamond Standard"},i=void 0,c={unversionedId:"smart-contracts/diamond",id:"version-1.6.0/smart-contracts/diamond",title:"Diamond Standard",description:"This example shows how you can use the implementation of diamond standard to implement diamond standard pattern for upgradeable and unlimited contracts.",source:"@site/versioned_docs/version-1.6.0/smart-contracts/diamond.md",sourceDirName:"smart-contracts",slug:"/smart-contracts/diamond",permalink:"/1.6.0/smart-contracts/diamond",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-1.6.0/smart-contracts/diamond.md",tags:[],version:"1.6.0",sidebarPosition:4,frontMatter:{sidebar_position:4,title:"Diamond Standard"},sidebar:"tutorialSidebar",previous:{title:"Proxy",permalink:"/1.6.0/smart-contracts/proxy"},next:{title:"Reentrancy Guard",permalink:"/1.6.0/smart-contracts/reentrancy-guard"}},d={},l=[{value:"Step 1: Include dependencies",id:"step-1-include-dependencies",level:2},{value:"Step 2: Add imports and enable unstable feature",id:"step-2-add-imports-and-enable-unstable-feature",level:2},{value:"Step 3: Define storage",id:"step-3-define-storage",level:2},{value:"Step 4: Inherit logic",id:"step-4-inherit-logic",level:2},{value:"Step 5: Define constructor",id:"step-5-define-constructor",level:2},{value:"Step 6: Define forward function",id:"step-6-define-forward-function",level:2},{value:"Step 6: Customize your contract",id:"step-6-customize-your-contract",level:2}],s={toc:l};function u(e){let{components:t,...n}=e;return(0,r.kt)("wrapper",(0,a.Z)({},s,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can use the implementation of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/contracts/upgradability/diamond"},"diamond standard")," to implement diamond standard pattern for upgradeable and unlimited contracts."),(0,r.kt)("h2",{id:"step-1-include-dependencies"},"Step 1: Include dependencies"),(0,r.kt)("p",null,"Include ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," as dependency in the cargo file or you can use ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml"))," template.\nAfter you need to enable default implementation of Diamond Standard via ",(0,r.kt)("inlineCode",{parentName:"p"},"brush")," features."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-toml"},'brush = { tag = "v1.6.0", git = "https://github.com/Supercolony-net/openbrush-contracts", default-features = false, features = ["diamond"] }\n')),(0,r.kt)("h2",{id:"step-2-add-imports-and-enable-unstable-feature"},"Step 2: Add imports and enable unstable feature"),(0,r.kt)("p",null,"Use ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contract")," macro instead of ",(0,r.kt)("inlineCode",{parentName:"p"},"ink::contract"),". Import ",(0,r.kt)("strong",{parentName:"p"},"everything")," from ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contracts::ownable")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"brush::contracts::diamond")),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[brush::contract]\npub mod my_diamond {\n    use brush::{\n        contracts::{\n            ownable::*,\n            diamond::*,\n        },\n        modifiers,\n    };\n...\n')),(0,r.kt)("h2",{id:"step-3-define-storage"},"Step 3: Define storage"),(0,r.kt)("p",null,"Declare storage struct and declare the field related to ",(0,r.kt)("inlineCode",{parentName:"p"},"DiamondStorage")," trait. Then you need to derive the ",(0,r.kt)("inlineCode",{parentName:"p"},"DiamondStorage")," trait and mark the corresponding field with the ",(0,r.kt)("inlineCode",{parentName:"p"},"#[DiamondStorageField]")," attribute. Deriving this trait allows you to reuse the default implementation of ",(0,r.kt)("inlineCode",{parentName:"p"},"DiamondStandard"),"."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"#[ink(storage)]\n#[derive(Default, SpreadAllocate, DiamondStorage)]\npub struct DiamondContract {\n    #[DiamondStorageField]\n    diamond: DiamondData,\n}\n")),(0,r.kt)("h2",{id:"step-4-inherit-logic"},"Step 4: Inherit logic"),(0,r.kt)("p",null,"Inherit implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"Diamond")," trait and of the ",(0,r.kt)("inlineCode",{parentName:"p"},"Ownable")," trait. You can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl Ownable for DiamondContract {}\n\nimpl Diamond for DiamondContract {}\n")),(0,r.kt)("h2",{id:"step-5-define-constructor"},"Step 5: Define constructor"),(0,r.kt)("p",null,"Define the constructor and initialize the owner with the contract initiator. Your basic version of ",(0,r.kt)("inlineCode",{parentName:"p"},"Diamond")," contract is ready!"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl DiamondContract {\n    #[ink(constructor)]\n    pub fn new(owner: AccountId, diamond_hash: Hash) -> Self {\n        ink_lang::codegen::initialize_contract(|instance: &mut Self| {\n            instance._init_with_owner(owner);\n            instance.diamond.self_hash = diamond_hash;\n        })\n    }\n}\n")),(0,r.kt)("h2",{id:"step-6-define-forward-function"},"Step 6: Define forward function"),(0,r.kt)("p",null,"Define the forward function to make delegate calls of facet contracts through the diamond contract."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl DiamondContract {\n    #[ink(message, payable, selector = _)]\n    pub fn forward(&self) {\n        self._fallback();\n    }\n}\n")),(0,r.kt)("h2",{id:"step-6-customize-your-contract"},"Step 6: Customize your contract"),(0,r.kt)("p",null,"You can add more basic functionality for your diamond contract by adding functions to ",(0,r.kt)("inlineCode",{parentName:"p"},"DiamondContract")," implemenation, but the point of the Diamond standard is not to increase the size of your contract, and to add upgradeable functionality to your contract via so called facets."),(0,r.kt)("p",null,"When you create a new contract (facet), which you want to make delegate calls from your diamond contract to, you will call the ",(0,r.kt)("inlineCode",{parentName:"p"},"diamond_cut")," function on your diamond contract, with the code hash of your new facet and the selectors of all the functions from this facet you want to use. The diamond will register them and anytime you call this function on your diamond contract, it will make the delegate call to the facet the function belongs to. You can add, remove or replace these functions anytime with the ",(0,r.kt)("inlineCode",{parentName:"p"},"diamond_cut")," function, some of the limitations are, that you can not add functions with the same selectors, when replacing functions, the new function needs to be from a different contract, then currently in use, and when removing functions, the function needs to be registered in the diamond contract."),(0,r.kt)("p",null,"You can check an example of the usage of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/Supercolony-net/openbrush-contracts/tree/main/examples/diamond"},"Diamond"),"."))}u.isMDXComponent=!0}}]);