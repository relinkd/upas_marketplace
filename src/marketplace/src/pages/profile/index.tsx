import { Typography, Stack } from '@mui/material';
import { Layout, Search } from 'widgets';
import { useQueryCall } from '@ic-reactor/react';
import { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { marketplaceModel } from 'entities/marketplace';
import { IssuerTuple } from 'shared';

export const ProfilePage = () => {

  const dispatch = useDispatch();

  const { data: issuers, call: refetchIssuers } = useQueryCall({
    functionName: "getIssuersBatch",
    args: [
      [],
      BigInt(10)
    ]
  })

  useEffect(() => {
    if(!issuers) return;
    
    dispatch(
      marketplaceModel.marketplaceActions.updateMarketplaceState({
        issuers: issuers as IssuerTuple[]
      })
    )
  }, [issuers])

  return (
    <Layout>
      <Stack flexDirection="row" justifyContent="center" width={1} maxWidth={1}>
        <Search />
      </Stack>
    </Layout>
  );
};
