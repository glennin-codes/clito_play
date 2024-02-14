import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, TouchableOpacity, View } from 'react-native';

export default function App() {
  return (
    <View className="relative flex-1  bg-black h-full" >
     <View className="w-full flex bg-gray-500 top-0 justify-centerNa   rounded-lg static p-4  py-6  ">
      <Text className="text-white ">My App</Text>
     </View>
   
     <View className="justify-center flex gap-6 items-center absolute  bottom-0 w-full">
        <TouchableOpacity className="p-4 bg-[#d6d3d1]  w-full  ">
          <Text className="text-black text-center text-lg font-bold py-2">
            Get started
          </Text>
        </TouchableOpacity>
      </View>
    
      <StatusBar style='light'/>
    </View>
  );
}
