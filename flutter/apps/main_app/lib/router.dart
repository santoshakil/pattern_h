import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:navigation/navigation.dart';

import 'features/home/presentation/home_screen.dart';

final routerProvider = Provider<GoRouter>(
  (ref) => createRouter(
    routes: [
      GoRoute(
        path: Routes.home,
        builder: (context, state) => const HomeScreen(),
      ),
    ],
  ),
);
