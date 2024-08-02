import { useEffect } from 'react';
import { useDispatch } from 'react-redux';
import { Stack, Typography } from '@mui/material';
import { userModel } from 'entities/user';
import { Layout, LayoutBackground } from 'widgets';
import { routes, GradientButtonWraper } from 'shared';
import { useNavigate } from 'react-router-dom';

export const ConnectPage = () => {
  return (
    <Layout bg={LayoutBackground.Sm}>
      <Typography variant="h1" className="center" mt={{ xs: 2, md: 15, lg: 36 }} mb={3.75}>
        Connect your wallet
      </Typography>
      <Stack direction={{ xs: 'column', md: 'row' }} spacing={1} justifyContent="center" alignItems="center">
      </Stack>
      <Typography className="center" mt={7.5}>
        Sign in by selecting your preffered wallet
      </Typography>
    </Layout>
  );
};
