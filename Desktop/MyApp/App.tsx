import { StatusBar } from "expo-status-bar";
import React from "react";
import { Text, View } from "react-native";
import { NavigationContainer } from "@react-navigation/native";
import { createNativeStackNavigator } from "@react-navigation/native-stack";
import HomeScreen from "./src/screens/Home/";
import VideoDisplayPage from "./src/screens/Video";

export default function App(): React.JSX.Element {
  const Stack = createNativeStackNavigator();
  return (
    <NavigationContainer>
      <Stack.Navigator initialRouteName="Home">
      <Stack.Screen name="Home" component={HomeScreen} options={{ headerShown: false }} />

        <Stack.Screen name="VideoSection" component={VideoDisplayPage} options={{ headerShown: false }} />
      </Stack.Navigator>

      <StatusBar style="light" />
    </NavigationContainer>
  );
}
