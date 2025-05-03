<template>
  <div class="register">
    <h1>Create Your Account</h1>
    <p class="subtitle">Join us and start collaborating</p>
    <form @submit.prevent="register">
      <div class="form-group">
        <label for="fullName">Full Name</label>
        <input type="text" id="fullName" v-model="fullName" placeholder="Enter your full name" required />
      </div>

      <div class="form-group">
        <label for="email">Email</label>
        <input type="email" id="email" v-model="email" placeholder="Enter your email" required />
      </div>

      <div class="form-group">
        <label for="workspaceName">Workspace Name</label>
        <input type="text" id="workspaceName" v-model="workspaceName" placeholder="Enter your workspace name"
          required />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password" v-model="password" placeholder="Enter your password" required />
      </div>

      <button type="submit" class="register-button">Register</button>
    </form>

    <p class="login-link">
      Already have an account? <router-link to="/login">Login here</router-link>.
    </p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      fullName: '',
      email: '',
      workspaceName: '',
      password: '',
    };
  },
  methods: {
    async register() {
      try {
        const user = await this.$store.dispatch('signup', {
          email: this.email,
          fullname: this.fullName,
          password: this.password,
          workspace: this.workspaceName,
        });

        console.log('Signup successful, user:', user);
        this.$router.push('/');
      } catch (error) {
        console.error('Register failed:', error);
      }
    },
  },
};
</script>

<style scoped>
.register {
  width: 600px;
  /* Similar to login view for consistency */
  margin: 120px auto;
  padding: 60px 50px;
  background-color: #ffffff;
  border-radius: 16px;
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.2);
  font-family: 'Inter', sans-serif;
  text-align: center;
}

h1 {
  margin-bottom: 20px;
  font-size: 36px;
  font-weight: 700;
  color: #2c3e50;
}

.subtitle {
  font-size: 18px;
  color: #7f8c8d;
  margin-bottom: 40px;
}

.form-group {
  margin-bottom: 30px;
  text-align: left;
}

label {
  display: block;
  margin-bottom: 12px;
  font-weight: 600;
  font-size: 16px;
  color: #34495e;
}

input {
  width: 100%;
  padding: 16px;
  border: 1px solid #dcdde1;
  border-radius: 10px;
  font-size: 18px;
  font-family: 'Inter', sans-serif;
  transition: all 0.3s ease;
}

input:focus {
  border-color: #3498db;
  box-shadow: 0 0 12px rgba(52, 152, 219, 0.4);
  outline: none;
}

.register-button {
  width: 100%;
  padding: 16px;
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 10px;
  font-size: 18px;
  cursor: pointer;
  font-weight: 600;
  transition: background-color 0.3s ease, transform 0.2s ease;
  margin-top: 30px;
}

.register-button:hover {
  background-color: #2980b9;
  transform: translateY(-3px);
  box-shadow: 0 6px 15px rgba(41, 128, 185, 0.4);
}

.register-button:active {
  transform: translateY(0);
  box-shadow: none;
}

.login-link {
  margin-top: 30px;
  font-size: 16px;
  color: #7f8c8d;
}

.login-link a {
  color: #3498db;
  text-decoration: none;
  font-weight: 600;
  transition: color 0.3s ease;
}

.login-link a:hover {
  color: #2980b9;
}
</style>
