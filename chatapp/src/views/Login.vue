<template>
  <div class="login">
    <h1>Welcome Back</h1>
    <p class="subtitle">Please login to your account</p>
    <form @submit.prevent="login">
      <div class="form-group">
        <label for="email">Email</label>
        <input type="email" id="email" v-model="email" placeholder="Enter your email" required />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input type="password" id="password" v-model="password" placeholder="Enter your password" required />
      </div>

      <button type="submit" class="login-button">Login</button>
    </form>

    <p class="register-link">
      Don't have an account? <router-link to="/register">Register here</router-link>.
    </p>
  </div>
</template>

<script>
export default {
  data() {
    return {
      email: '',
      password: '',
    };
  },
  methods: {
    async login() {
      try {
        const user = await this.$store.dispatch('signin', {
          email: this.email,
          password: this.password,
        });

        console.log('Signin successful, user:', user);
        this.$router.push('/');
      } catch (error) {
        console.error('Signin failed:', error);
      }
    },
  },
};
</script>

<style scoped>
.login {
  width: 600px;
  /* Increased width for a larger form */
  margin: 120px auto;
  /* Increased margin for more space around the form */
  padding: 60px 50px;
  /* Increased padding for a more spacious feel */
  background-color: #ffffff;
  border-radius: 16px;
  /* Larger border-radius for a softer, premium look */
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.2);
  /* Deeper shadow for more depth */
  font-family: 'Inter', sans-serif;
  text-align: center;
}

h1 {
  margin-bottom: 20px;
  font-size: 36px;
  /* Larger font size for a more impactful heading */
  font-weight: 700;
  color: #2c3e50;
}

.subtitle {
  font-size: 18px;
  /* Slightly larger subtitle */
  color: #7f8c8d;
  margin-bottom: 40px;
  /* More space between subtitle and form */
}

.form-group {
  margin-bottom: 30px;
  /* Increased margin between form groups */
  text-align: left;
}

label {
  display: block;
  margin-bottom: 12px;
  /* More space between label and input */
  font-weight: 600;
  font-size: 16px;
  /* Larger label font size */
  color: #34495e;
}

input {
  width: 100%;
  padding: 16px;
  /* Larger padding for inputs */
  border: 1px solid #dcdde1;
  border-radius: 10px;
  /* Slightly larger border-radius for inputs */
  font-size: 18px;
  /* Larger input font size */
  font-family: 'Inter', sans-serif;
  transition: all 0.3s ease;
}

input:focus {
  border-color: #3498db;
  box-shadow: 0 0 12px rgba(52, 152, 219, 0.4);
  /* Larger shadow on focus */
  outline: none;
}

.login-button {
  width: 100%;
  padding: 16px;
  /* Larger padding for the button */
  background-color: #3498db;
  color: white;
  border: none;
  border-radius: 10px;
  font-size: 18px;
  /* Larger button font size */
  cursor: pointer;
  font-weight: 600;
  transition: background-color 0.3s ease, transform 0.2s ease;
  margin-top: 30px;
  /* More space above the button */
}

.login-button:hover {
  background-color: #2980b9;
  transform: translateY(-3px);
  /* Increased hover effect */
  box-shadow: 0 6px 15px rgba(41, 128, 185, 0.4);
  /* Deeper shadow on hover */
}

.login-button:active {
  transform: translateY(0);
  box-shadow: none;
}

.register-link {
  margin-top: 30px;
  /* More space above the register link */
  font-size: 16px;
  /* Larger font size for the link */
  color: #7f8c8d;
}

.register-link a {
  color: #3498db;
  text-decoration: none;
  font-weight: 600;
  /* Bolder font for the link */
  transition: color 0.3s ease;
}

.register-link a:hover {
  color: #2980b9;
}
</style>
