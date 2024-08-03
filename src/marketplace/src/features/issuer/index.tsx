import { Stack, styled, Typography } from '@mui/material';
import { IssuerTuple, GradientButtonWraper, BORDER_RADIUS_S, COLOR_BLACK } from 'shared';
import icon from './assets/icon.png'

const StyledImg = styled('img')({
  display: 'block',
  width: '40px',
  height: '40px'
});

export const Issuer = ({ issuer }: { issuer: IssuerTuple }) => {
  return (
    <GradientButtonWraper width={1} sx={{
      borderRadius: BORDER_RADIUS_S,
    }}>
      <Stack flexDirection="row" alignItems="center" justifyContent="space-between" width={1} paddingX={2} height="70px" sx={{
        backgroundColor: 'white',
        borderRadius: BORDER_RADIUS_S
      }}>
          <Stack flexDirection="row" alignItems="center">
            <StyledImg src={icon} />
            <Typography color={COLOR_BLACK} paddingLeft={6}>{issuer[1].name}</Typography>
          </Stack>
          <Typography>{issuer[1].verified ? 'verified' : 'unverified'}</Typography>
      </Stack>
    </GradientButtonWraper>
  );
};
