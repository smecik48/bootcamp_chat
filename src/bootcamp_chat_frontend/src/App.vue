<script lang="ts">
import { ref } from 'vue';
import { bootcamp_chat_backend, canisterId, createActor } from '../../declarations/bootcamp_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import type { UserData } from '../../declarations/bootcamp_chat_backend/bootcamp_chat_backend.did';

export default {
  data() {
    return {
      newChat: "",
      chats: [] as string[][],
      identity: undefined as undefined | Identity,
      principal: undefined as undefined | Principal,
      targetPrincipal: "",
      userData: undefined as undefined | UserData,
      newUserName: "",
      allUsers: [] as [Principal, UserData][]
    }
  },
  methods: {
    isUserLogged() {
      if (!this.identity || !this.principal || this.principal === Principal.anonymous()){
        throw new Error("Log in man")
      }
      return{
        identity: this.identity,
        principal: this.principal
      }
    },
    validateTargetPrincipal(){
      const cleanTargetPrincipal = this.targetPrincipal.trim()
      if (cleanTargetPrincipal === ""){
        throw new Error("Principal not given")
      }

      const targetPrincipal = Principal.fromText(cleanTargetPrincipal)
      if (!targetPrincipal || targetPrincipal === Principal.anonymous()){
        throw new Error("Wrong target")
      }
      return targetPrincipal
    },
    getAuthClient(){
      this.isUserLogged()
      return createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }
      })
    },
    async dodajChat() {
      const targetPrincipal = this.validateTargetPrincipal()

      const backend = this.getAuthClient()

      await backend.add_chat_msg(this.newChat, targetPrincipal)
      
      await this.pobierzChaty()
    },
    async pobierzChaty() {
      const {identity, principal} = this.isUserLogged();
      const targetPrincipal = this.validateTargetPrincipal()

      const chatPath = [identity.getPrincipal(), targetPrincipal].sort()
      this.chats = await bootcamp_chat_backend.get_chat(chatPath)
    },
    async login() {
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://ajuq4-ruaaa-aaaaa-qaaga-cai.localhost:4943",
        onSuccess: async () => {
          const identity = authClient.getIdentity();
          const principal = identity.getPrincipal()
          this.principal = principal
          
          console.log("Zalogowano", identity.getPrincipal())
          this.identity = identity
          await this.getUserData()
          await this.getAllUsers()
        }
      })

      
      
    },
    async logout(){
      const authClient = await AuthClient.create()
      await authClient.logout()
      this.identity = undefined
      this.principal = undefined
      this.chats = []
      this.userData = undefined
    },
    async registerUserName(){
        const trimedUserName = this.newUserName.trim()
        const backend = this.getAuthClient()
        await backend.register(trimedUserName)
        await this.getUserData()
        await this.getAllUsers()

    },
    async getUserData(){
      const {identity, principal} = this.isUserLogged()
      const maybeUserData = await bootcamp_chat_backend.get_user(principal as Principal)
          if (maybeUserData.length === 0){
            this.userData = undefined
          } else {
            this.userData = maybeUserData[0]
          }
          console.log("user data", this.userData)
    },
    async getAllUsers(){
      this.allUsers = await bootcamp_chat_backend.get_users()
    }
  }
}
</script>

<template>
  <main>
    <button v-if="!principal" @click="login">Login</button>
    <button v-if="principal" @click="logout">Logout</button>
    <div v-if="principal && !userData">
      <input v-model="newUserName" placeholder="Nick"/> <button @click="registerUserName">Register</button>
    </div>
    <div v-if="principal && userData">
      <div>
        <div v-if="allUsers">
          <select v-model="targetPrincipal">
            <option v-for="[userPrincipal, userData] in allUsers" :value="userPrincipal.toText()"> {{userData.nickname}}</option>
          </select>
        </div>
        <div v-for="chat in chats[0]">
          {{ chat }}
        </div>
        </div>
        <div>
          <textarea v-model="newChat" placeholder="Wiadomość"></textarea><button @click="dodajChat">Dodaj notatke</button>
        </div>
    </div>
  </main>
</template>