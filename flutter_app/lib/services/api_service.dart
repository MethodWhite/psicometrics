import 'dart:convert';
import 'dart:math';
import 'package:crypto/crypto.dart';
import 'package:dio/dio.dart';
import '../config/api.dart';
import '../models/test_info.dart';
import '../models/question.dart';
import '../models/results.dart';

class ApiService {
  static final ApiService _instance = ApiService._();
  factory ApiService() => _instance;

  late final Dio _dio;
  String _deviceId = '';
  String _sessionToken = '';

  ApiService._() {
    _deviceId = _generateDeviceId();
    _dio = Dio(BaseOptions(
      baseUrl: ApiConfig.baseUrl,
      connectTimeout: ApiConfig.timeout,
      receiveTimeout: ApiConfig.timeout,
      headers: {'Content-Type': 'application/json'},
    ));

    // Interceptor for Zero Trust challenge-response
    _dio.interceptors.add(InterceptorsWrapper(
      onRequest: (options, handler) async {
        if (options.method == 'POST') {
          try {
            // Get challenge from server
            final challengeResponse = await _dio.get('/api/v1/auth/challenge', queryParameters: {
              'session_id': _sessionToken,
              'agent_type': 'flutter',
            });
            final challenge = challengeResponse.data;
            final nonce = challenge['nonce'] as String;
            final challengeId = challenge['challenge_id'] as String;

            // Compute HMAC-SHA256 response
            final response = _computeHmacResponse(nonce);

            // Add Zero Trust headers
            options.headers['X-Challenge-Id'] = challengeId;
            options.headers['X-Challenge-Response'] = response;
            options.headers['X-Device-Id'] = _deviceId;
          } catch (_) {
            // Challenge failed, proceed without Zero Trust (degraded mode)
          }
        }
        handler.next(options);
      },
    ));
  }

  String _generateDeviceId() {
    final random = Random.secure();
    final bytes = List<int>.generate(16, (_) => random.nextInt(256));
    return base64Url.encode(bytes);
  }

  String _computeHmacResponse(String nonce) {
    final key = utf8.encode('psicometrics-device-$_deviceId');
    final data = utf8.encode(nonce);
    final hmacSha256 = Hmac(sha256, key);
    final digest = hmacSha256.convert(data);
    return digest.toString();
  }

  Future<void> initSession() async {
    try {
      // Register device with server
      final nonce = base64Url.encode(List<int>.generate(16, (_) => Random.secure().nextInt(256)));
      final signedNonce = _computeHmacResponse(nonce);

      final response = await _dio.post('/api/v1/auth/device/register', data: {
        'device_id': _deviceId,
        'platform': 'android',
        'signed_nonce': signedNonce,
        'app_version': '1.0.0',
      });

      if (response.data['trusted'] == false && response.data['challenge'] != null) {
        // Attest with the challenge
        final challenge = response.data['challenge'] as String;
        final attestResponse = _computeHmacResponse(challenge);

        await _dio.post('/api/v1/auth/device/attest', data: {
          'device_id': _deviceId,
          'platform': 'android',
          'signed_nonce': attestResponse,
          'app_version': '1.0.0',
        });
      }
    } catch (_) {
      // Device registration failed, proceed without attestation
    }
  }

  Future<List<TestInfo>> getTests() async {
    final response = await _dio.get('/api/v1/tests');
    return (response.data as List).map((t) => TestInfo.fromJson(t)).toList();
  }

  Future<TestData> getTestQuestions(String testType, {String lang = 'es'}) async {
    final response = await _dio.get('/api/v1/tests/$testType', queryParameters: {'lang': lang});
    return TestData.fromJson(response.data);
  }

  Future<dynamic> submitBigFive(List<Map<String, dynamic>> answers, String lang) async {
    final response = await _dio.post('/api/v1/tests/big_five/submit', data: {
      'answers': answers,
      'language': lang,
    });
    return BigFiveResult.fromJson(response.data);
  }

  Future<dynamic> submitMBTI(List<Map<String, dynamic>> answers, String lang) async {
    final response = await _dio.post('/api/v1/tests/mbti/submit', data: {
      'answers': answers,
      'language': lang,
    });
    return MBTIResult.fromJson(response.data);
  }

  Future<dynamic> submitEnneagram(List<Map<String, dynamic>> answers, String lang) async {
    final response = await _dio.post('/api/v1/tests/enneagram/submit', data: {
      'answers': answers,
      'language': lang,
    });
    return EnneagramResult.fromJson(response.data);
  }

  Future<dynamic> submitDISC(List<Map<String, dynamic>> answers, String lang) async {
    final response = await _dio.post('/api/v1/tests/disc/submit', data: {
      'answers': answers,
      'language': lang,
    });
    return DISCResult.fromJson(response.data);
  }

  Future<dynamic> submitDarkTriad(List<Map<String, dynamic>> answers, String lang) async {
    final response = await _dio.post('/api/v1/tests/dark_triad/submit', data: {
      'answers': answers,
      'language': lang,
    });
    return DarkTriadResult.fromJson(response.data);
  }

  Future<dynamic> submitHumanDesign(String birthDate, String birthTime, String birthLocation, String lang) async {
    final response = await _dio.post('/api/v1/tests/human_design/submit', data: {
      'birth_date': birthDate,
      'birth_time': birthTime,
      'birth_location': birthLocation,
      'language': lang,
    });
    return HumanDesignResult.fromJson(response.data);
  }
}
