import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

typedef AuthChecker = bool Function();

class AuthGuard {
  final AuthChecker isAuthenticated;
  final String loginPath;

  const AuthGuard({required this.isAuthenticated, this.loginPath = '/login'});

  String? redirect(BuildContext context, GoRouterState state) {
    if (!isAuthenticated() && state.uri.path != loginPath) {
      return loginPath;
    }
    return null;
  }
}
