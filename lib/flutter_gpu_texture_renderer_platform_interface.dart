import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'flutter_gpu_texture_renderer_method_channel.dart';

abstract class FlutterGpuTextureRendererPlatform extends PlatformInterface {
  /// Constructs a FlutterGpuTextureRendererPlatform.
  FlutterGpuTextureRendererPlatform() : super(token: _token);

  static final Object _token = Object();

  static FlutterGpuTextureRendererPlatform _instance = MethodChannelFlutterGpuTextureRenderer();

  /// The default instance of [FlutterGpuTextureRendererPlatform] to use.
  ///
  /// Defaults to [MethodChannelFlutterGpuTextureRenderer].
  static FlutterGpuTextureRendererPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [FlutterGpuTextureRendererPlatform] when
  /// they register themselves.
  static set instance(FlutterGpuTextureRendererPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
