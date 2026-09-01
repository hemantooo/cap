import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { DeviceIdentity } from './types';

export function useDeviceAuth() {
  const [deviceIdentity, setDeviceIdentity] = useState<DeviceIdentity | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  const authenticateDevice = async () => {
    setLoading(true);
    setError(null);
    try {
      const fingerprint = await invoke<string>('get_device_fingerprint');
      
      const platform = navigator.userAgent.toLowerCase();
      let platformName = 'unknown';
      if (platform.includes('win')) platformName = 'windows';
      else if (platform.includes('mac')) platformName = 'macos';
      else if (platform.includes('linux')) platformName = 'linux';

      setDeviceIdentity({
        fingerprint,
        platform: platformName,
        timestamp: Date.now()
      });
    } catch (err) {
      if (err instanceof Error) {
        setError(err.message);
      } else if (typeof err === 'string') {
        setError(err);
      } else {
        setError('An unknown error occurred while authenticating device.');
      }
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    authenticateDevice();
  }, []);

  return { deviceIdentity, loading, error, authenticateDevice };
}
