trait DelegationRegistry {
  /// WRITE ///
  pub fn delegateForAll(&mut self, delegate: string, value: bool);
  pub fn delegateForContract(&mut self, delegate: string, _contract: string, value: bool);
  pub fn delegateForToken(&mut self, delegate: string, _contract: string, tokenId: u128, value: bool);
  pub fn revokeAllDelegates(&mut self) external;
  pub fn revokeDelegate(&mut self, delegate: string);
  pub fn revokeSelf(&mut self, vault: string);

  /// READ ///
  fn getDelegationsByDelegate(self, delegate: string) -> DelegationInfo[];
  fn getDelegatesForAll(self, vault: string) -> address[];
  fn getDelegatesForContract(self, vault: string, contract_: string) -> address[];
  fn getDelegatesForToken(self, vault: string, contract_: string, tokenId: u128) -> address[];
  fn checkDelegateForAll(self, delegate: string, vault: string) -> bool;
  fn checkDelegateForContract(self, delegate: string, vault: string, contract_: string) -> bool;
  fn checkDelegateForToken(self, delegate: string, vault: string, contract_: string, tokenId: u128) -> bool;
}
