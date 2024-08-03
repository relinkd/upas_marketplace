import { Stack, Link, styled } from '@mui/material';
import { COLOR_LIGHT_GRAY, BORDER_RADIUS_M, COLOR_TEXT_GRAY, BORDER, COLOR_BLACK } from 'shared';

export const FooterLink = styled(Link)(({ theme }) => ({
    paddingLeft: 8,
    paddingRight: 8,
    color: COLOR_BLACK,
    fontSize: "20px"
}))


export const Footer = () => {
  return (
    <Stack direction="row" sx={{
        width: '500px',
        height: '65px',
        margin: '0 auto',
        backgroundColor: COLOR_LIGHT_GRAY,
        borderRadius: BORDER_RADIUS_M,
        alignItems: 'center',
        justifyContent: 'space-around',
        border: BORDER,
        borderColor: COLOR_TEXT_GRAY,
        marginBottom: 2
    }}>
      <FooterLink href="#">Github</FooterLink>
      <FooterLink href="#">DOCs</FooterLink>
    </Stack>
  )
};
