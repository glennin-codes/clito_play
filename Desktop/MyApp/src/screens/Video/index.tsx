import React, { useState } from 'react';
import { View, Text, TouchableOpacity, Modal, ActivityIndicator } from 'react-native';
import { Video,ResizeMode } from 'expo-av';
import { AntDesign } from '@expo/vector-icons';

const VideoPlayer = () => {
  const [isVideoLoading, setIsVideoLoading] = useState(true);
  const [isErrorModalVisible, setIsErrorModalVisible] = useState(false);
  const videoUri = 'https://mediaglens.s3.amazonaws.com/94d0479c8d07/videos/75260fdc-9a11-4cdb-b5cf-aca0ecde3a0f.mp4';

  const handlePlaybackStatusUpdate = (playbackStatus) => {
    if (!playbackStatus.isLoaded && !playbackStatus.isPlaying && playbackStatus.error) {
      setIsErrorModalVisible(true);
    } else if (playbackStatus.isBuffering) {
      setIsVideoLoading(true);
    } else {
      setIsVideoLoading(false);
    }
  };

  const handleRetry = () => {
    setIsErrorModalVisible(false);
    // You can implement your retry logic here
  };

  return (
    <View className='flex-1 justify-center items-center bg-black'>
      {isVideoLoading && (
        <View className='absolute inset-0 justify-center items-center bg-black'>
          <ActivityIndicator size="large" color="white" />
        </View>
      )}
      <Video
        source={{ uri: videoUri }}
        className= 'w-full h-auto aspect-[16/9]'
        useNativeControls
        resizeMode={ 'contain' as ResizeMode}
        isLooping
        shouldPlay
        onPlaybackStatusUpdate={handlePlaybackStatusUpdate}
        onLoad={() => setIsVideoLoading(false)}
      />
      <Modal
        animationType="slide"
        transparent={true}
        visible={isErrorModalVisible}
        onRequestClose={() => setIsErrorModalVisible(false)}
      >
        <View className='flex-1 justify-center items-center bg-black bg-opacity-50'>
          <View className='bg-white rounded-lg p-4'>
          <AntDesign name="exclamationcircle" size={24} color="red" className='mb-4' />
            <Text className='text-lg font-bold mb-4 text-center'>
              An error occurred while loading the video.
            </Text>
            <TouchableOpacity className='bg-blue-500 rounded p-2' onPress={handleRetry}>
              <Text className='text-white font-bold'>Retry</Text>
            </TouchableOpacity>
          </View>
        </View>
      </Modal>
    </View>
  );
};

export default VideoPlayer;
