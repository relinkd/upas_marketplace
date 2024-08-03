import { Typography, Stack } from '@mui/material';
import { Layout, Search } from 'widgets';
import { useQueryCall } from '@ic-reactor/react';

export const ProfilePage = () => {

  const { data: count, call: refetchCount } = useQueryCall({
    functionName: "getIssuersBatch",
    args: [
      [],
      BigInt(10)
    ]
  })

  console.log(count)

  return (
    <Layout>
      <Stack flexDirection="row" justifyContent="center" width={1} maxWidth={1}>
        <Search />
      </Stack>
    </Layout>
  );
};
