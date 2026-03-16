import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import 'routes.dart';

GoRouter createRouter({required List<RouteBase> routes, String? initial}) =>
    GoRouter(initialLocation: initial ?? Routes.home, routes: routes);

final routerProvider = Provider<GoRouter>(
  (ref) => throw UnimplementedError('Override routerProvider in app'),
);
