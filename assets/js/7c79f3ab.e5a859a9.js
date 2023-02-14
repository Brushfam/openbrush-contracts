"use strict";(self.webpackChunkopenbrush=self.webpackChunkopenbrush||[]).push([[79269],{3905:(e,t,n)=>{n.d(t,{Zo:()=>c,kt:()=>f});var a=n(67294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function s(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function i(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),l=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):s(s({},t),e)),n},c=function(e){var t=l(e.components);return a.createElement(p.Provider,{value:t},e.children)},u="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,p=e.parentName,c=i(e,["components","mdxType","originalType","parentName"]),u=l(n),d=r,f=u["".concat(p,".").concat(d)]||u[d]||m[d]||o;return n?a.createElement(f,s(s({ref:t},c),{},{components:n})):a.createElement(f,s({ref:t},c))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,s=new Array(o);s[0]=d;var i={};for(var p in t)hasOwnProperty.call(t,p)&&(i[p]=t[p]);i.originalType=e,i[u]="string"==typeof e?e:r,s[1]=i;for(var l=2;l<o;l++)s[l]=n[l];return a.createElement.apply(null,s)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},44130:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>s,default:()=>m,frontMatter:()=>o,metadata:()=>i,toc:()=>l});var a=n(87462),r=(n(67294),n(3905));const o={sidebar_position:6,title:"PSP22 Pausable"},s=void 0,i={unversionedId:"smart-contracts/PSP22/Extensions/pausable",id:"version-3.0.0-beta/smart-contracts/PSP22/Extensions/pausable",title:"PSP22 Pausable",description:"This example shows how you can implement a PSP22 contract with a Pausable extension. See an example of PSP22Pausable implementation.",source:"@site/versioned_docs/version-3.0.0-beta/smart-contracts/PSP22/Extensions/pausable.md",sourceDirName:"smart-contracts/PSP22/Extensions",slug:"/smart-contracts/PSP22/Extensions/pausable",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/Extensions/pausable",draft:!1,editUrl:"https://github.com/727-Ventures/openbrush-contracts/tree/main/docs/versioned_docs/version-3.0.0-beta/smart-contracts/PSP22/Extensions/pausable.md",tags:[],version:"3.0.0-beta",sidebarPosition:6,frontMatter:{sidebar_position:6,title:"PSP22 Pausable"},sidebar:"tutorialSidebar",previous:{title:"PSP22 FlashMint",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/Extensions/flashmint"},next:{title:"PSP22 Capped",permalink:"/openbrush-contracts/3.0.0-beta/smart-contracts/PSP22/Extensions/capped"}},p={},l=[{value:"Step 1: Import default implementation",id:"step-1-import-default-implementation",level:2},{value:"Step 2: Inherit logic and apply <code>when_not_paused</code> modifier",id:"step-2-inherit-logic-and-apply-when_not_paused-modifier",level:2},{value:"Step 3: Define constructor",id:"step-3-define-constructor",level:2},{value:"Step 4: Customize your contract with <code>Pausable</code> logic",id:"step-4-customize-your-contract-with-pausable-logic",level:2},{value:"Final code:",id:"final-code",level:2}],c={toc:l},u="wrapper";function m(e){let{components:t,...n}=e;return(0,r.kt)(u,(0,a.Z)({},c,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("p",null,"This example shows how you can implement a ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/token/psp22"},"PSP22")," contract with a ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/contracts/src/security/pausable"},"Pausable")," extension. See an example of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp22_extensions/pausable"},"PSP22Pausable")," implementation."),(0,r.kt)("p",null,"First, you should implement basic version of ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22"),"."),(0,r.kt)("h2",{id:"step-1-import-default-implementation"},"Step 1: Import default implementation"),(0,r.kt)("p",null,"With ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#the-default-toml-of-your-project-with-openbrush"},"default ",(0,r.kt)("inlineCode",{parentName:"a"},"Cargo.toml")),",\nyou need to import the ",(0,r.kt)("inlineCode",{parentName:"p"},"psp22")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"pausable")," modules, enable corresponding features, and embed modules data structures\nas described in ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/overview#reuse-implementation-of-traits-from-openbrush"},"that section"),"."),(0,r.kt)("p",null,"The main trait is ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable"),"."),(0,r.kt)("h2",{id:"step-2-inherit-logic-and-apply-when_not_paused-modifier"},"Step 2: Inherit logic and apply ",(0,r.kt)("inlineCode",{parentName:"h2"},"when_not_paused")," modifier"),(0,r.kt)("p",null,"Inherit the implementation of the ",(0,r.kt)("inlineCode",{parentName:"p"},"PSP22")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"Pausable")," traits.\nYou can customize (override) methods in this ",(0,r.kt)("inlineCode",{parentName:"p"},"impl")," block. We will apply the\n",(0,r.kt)("inlineCode",{parentName:"p"},"when_not_paused")," modifier for the transfer."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl PSP22 for Contract {}\n\nimpl Transfer for Contract {\n    /// Return `Paused` error if the token is paused\n    #[modifiers(when_not_paused)]\n    fn _before_token_transfer(\n        &mut self,\n        _from: Option<&AccountId>,\n        _to: Option<&AccountId>,\n        _amount: &Balance,\n    ) -> Result<(), PSP22Error> {\n        // TODO logic for before token transfer\n        Ok(())\n    }\n}\n\nimpl Pausable for Contract {}\n")),(0,r.kt)("h2",{id:"step-3-define-constructor"},"Step 3: Define constructor"),(0,r.kt)("p",null,"Define constructor and add contract functions for pausing and unpausing the contract."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl Contract {\n    #[ink(constructor)]\n    pub fn new(total_supply: Balance) -> Self {\n        let mut instance = Self::default();\n\n        assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());\n        \n        instance\n    }\n}\n")),(0,r.kt)("h2",{id:"step-4-customize-your-contract-with-pausable-logic"},"Step 4: Customize your contract with ",(0,r.kt)("inlineCode",{parentName:"h2"},"Pausable")," logic"),(0,r.kt)("p",null,"Add the ",(0,r.kt)("inlineCode",{parentName:"p"},"change_state")," function that allow switch pause state."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},"impl Contract {\n    ...\n    \n    /// Function which changes state to unpaused if paused and vice versa\n    #[ink(message)]\n    pub fn change_state(&mut self) -> Result<(), PSP22Error> {\n        if self.paused() {\n            self._unpause()\n        } else {\n            self._pause()\n        }\n    }\n}\n")),(0,r.kt)("h2",{id:"final-code"},"Final code:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust"},'#![cfg_attr(not(feature = "std"), no_std)]\n#![feature(min_specialization)]\n\n#[openbrush::contract]\npub mod my_psp22_pausable {\n    use openbrush::{\n        contracts::{\n            pausable::*,\n            psp22::*,\n        },\n        modifiers,\n        traits::Storage,\n    };\n\n    #[ink(storage)]\n    #[derive(Default, Storage)]\n    pub struct Contract {\n        #[storage_field]\n        psp22: psp22::Data,\n        #[storage_field]\n        pause: pausable::Data,\n    }\n\n    impl PSP22 for Contract {}\n\n    impl Transfer for Contract {\n        /// Return `Paused` error if the token is paused\n        #[modifiers(when_not_paused)]\n        fn _before_token_transfer(\n            &mut self,\n            _from: Option<&AccountId>,\n            _to: Option<&AccountId>,\n            _amount: &Balance,\n        ) -> Result<(), PSP22Error> {\n            // TODO logic for before token transfer\n            Ok(())\n        }\n    }\n\n    impl Pausable for Contract {}\n\n    impl Contract {\n        #[ink(constructor)]\n        pub fn new(total_supply: Balance) -> Self {\n            let mut instance = Self::default();\n\n            assert!(instance._mint_to(Self::env().caller(), total_supply).is_ok());\n            \n            instance\n        }\n\n        /// Function which changes state to unpaused if paused and vice versa\n        #[ink(message)]\n        pub fn change_state(&mut self) -> Result<(), PSP22Error> {\n            if self.paused() {\n                self._unpause()\n            } else {\n                self._pause()\n            }\n        }\n    }\n}\n')),(0,r.kt)("p",null,"You can check an implementation example of ",(0,r.kt)("a",{parentName:"p",href:"https://github.com/727-Ventures/openbrush-contracts/tree/main/examples/psp22_extensions/pausable"},"PSP22 Pausable"),"."),(0,r.kt)("p",null,"You can also check the documentation for the basic implementation of ",(0,r.kt)("a",{parentName:"p",href:"/smart-contracts/PSP22"},"PSP22"),"."))}m.isMDXComponent=!0}}]);