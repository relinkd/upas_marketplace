import { ProfilePage } from 'pages/profile';

import { RoutesProps } from './types';

export const appRoutes: RoutesProps = {
  home: {
    path: '/profile',
    component: <ProfilePage />,
  },
};
