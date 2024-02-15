import { useNavigation } from "@react-navigation/native";
import { NativeStackNavigationProp } from "@react-navigation/native-stack";
import React, { useEffect, useRef } from "react";
import { RootStackParamList } from "../../types";
import { Animated, Easing,  TouchableOpacity, View ,Text, Image} from 'react-native';
// import { NativeWindStyleSheet } from "nativewind";
function HomeScreen() {
  const navigation =
    useNavigation<NativeStackNavigationProp<RootStackParamList>>();
   
    const rotateAnim = useRef(new Animated.Value(0)).current;

    useEffect(() => {
      Animated.loop(
        Animated.timing(
          rotateAnim,
          {
            toValue:  1,
            duration:  2000,
            easing: Easing.linear,
            useNativeDriver: true,
          }
        )
      ).start();
    }, [rotateAnim]);
// NativeWindStyleSheet.setOutput({
//   default: "native",
// });

  return (
    <View className="flex-1  flex-col justify-center bg-black h-full">
      
    <View className="justify-center w-full text-red flex items-center ">
    <Animated.Image
        source={require('./image/home.png')}
        style={{
          width: 200,
          height: 200,
          backgroundColor:"red",
         
          transform: [
            {
              rotate: rotateAnim.interpolate({
                inputRange: [0, 360],
                outputRange: ['0deg', '360deg'],
              }),
            },
          ],
        }}
      />
    </View>
      <View className="justify-center flex gap-6 items-center absolute  bottom-0 w-full">
       
        <TouchableOpacity
          className="p-4 bg-[#d6d3d1]   w-full "
          onPress={() => navigation.navigate("VideoSection")}
        >
          <Text className="text-black text-center text-lg font-bold py-2">
            Get started
          </Text>
        </TouchableOpacity>
      </View>
    </View>
  );
}

export default HomeScreen;
