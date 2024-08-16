import { Typography, Stack, Button } from '@mui/material';
import { Layout, Search } from 'widgets';
import { useQueryCall, useAuth, useAgent } from '@ic-reactor/react';
import { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { marketplaceModel } from 'entities/marketplace';
import { IssuerTuple } from 'shared';
import { Issuers } from 'widgets/issuers';

export const MarketplacePage = () => {

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
      <Stack flexDirection="column" alignItems="center" width={1} maxWidth={1}>
        <Search />
        <Issuers />
      </Stack>
    </Layout>
  );
};
