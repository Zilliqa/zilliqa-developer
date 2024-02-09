export default {
  methods: {
    __getZilPay() {
      if (typeof window['zilPay'] !== 'undefined') {
        return Promise.resolve(window['zilPay'])
      }

      return new Promise((resolve, reject) => {
        let k = 0;

        const i = setInterval(() => {
          if (k >= 10) {
            clearInterval(i)

            return reject('ZilPay inot installed')
          }

          if (typeof window['zilPay'] !== 'undefined') {
            clearInterval(i)

            return resolve(window['zilPay'])
          }

          k++;
        }, 100);
      });
    }
  }
};
