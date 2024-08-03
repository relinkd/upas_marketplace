import { MarketplacePage } from 'pages/marketplace';

import { RoutesProps } from './types';

export const appRoutes: RoutesProps = {
  home: {
    path: '/',
    component: <MarketplacePage />,
  },
};
