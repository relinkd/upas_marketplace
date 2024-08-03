import { Box } from '@mui/material';
import { useShallowSelector } from 'shared';
import { marketplaceModel } from 'entities/marketplace';
import { Issuer } from 'features/issuer';

export const Issuers = () => {

    const { issuers } = useShallowSelector(marketplaceModel.selectors.getMarketplace);

  return (
    <Box paddingTop={12} sx={{
      width: '100%',
      display: 'flex'
    }}>
        {
          issuers.map((issuer) => <Issuer issuer={issuer}/>)
        }
    </Box>
  );
};
