import { FC, PropsWithChildren } from 'react';
import { ActorProvider, AgentProvider } from '@ic-reactor/react';
import { idlFactory, canisterId } from '../../../../declarations/indexer';

export const WithICConnect: FC<PropsWithChildren> = ({ children }) => {
  return (
  <AgentProvider withProcessEnv host='http://127.0.0.1:4943'>
    <ActorProvider idlFactory={idlFactory} canisterId={canisterId}>
      {children}
    </ActorProvider>
  </AgentProvider>
  );
};
