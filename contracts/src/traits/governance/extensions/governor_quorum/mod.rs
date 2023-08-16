// Copyright (c) 2023 Brushfam
// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::traits::errors::GovernanceError;
use openbrush::traits::Timestamp;

#[openbrush::trait_definition]
pub trait Quorum {
    #[ink(message)]
    fn quorum_numerator(&self) -> u128;

    #[ink(message)]
    fn quorum_numerator_at(&self, time_point: Timestamp) -> u128;

    #[ink(message)]
    fn quorum_denominator(&self) -> u128;

    #[ink(message)]
    fn quorum(&self, time_point: Timestamp) -> Result<u128, GovernanceError>;

    #[ink(message)]
    fn update_quorum_numerator(&mut self, numerator: u128) -> Result<(), GovernanceError>;
}

#[openbrush::wrapper]
pub type QuorumRef = dyn Quorum;
