import { Typography, Stack } from '@mui/material';
import { Layout } from 'widgets';
import { useQueryCall } from '@ic-reactor/react';

export const ProfilePage = () => {

  const { data: count, call: refetchCount } = useQueryCall({
    functionName: "getIssuersBatch",
    args: [{ batch: 10}],
  })

  return (
    <Layout>
      <Stack flexDirection="row" justifyContent="center" width={1} maxWidth={1}>
        
      </Stack>
    </Layout>
  );
};
