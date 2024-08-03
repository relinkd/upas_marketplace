import { Stack, styled, Typography } from '@mui/material';
import { IssuerTuple } from 'shared';
import icon from './assets/icon.png'

const StyledImg = styled('img')({
  display: 'block',
  width: '40px',
  height: '40px'
});

export const Issuer = ({ issuer }: { issuer: IssuerTuple }) => {
  return (
    <Stack flexDirection="row" alignItems="center" justifyContent="space-between" width={1} paddingX={2} height="70px">
        <StyledImg src={icon} />
        <Typography>{issuer[1].name}</Typography>
        <Typography>{issuer[1].verified ? 'verified' : 'unverified'}</Typography>
    </Stack>
  );
};
