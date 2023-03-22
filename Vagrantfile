Vagrant.configure("2") do |config|
  config.nfs.functional = false
  config.vm.synced_folder "./", "/vagrant", type: "virtiofs"

  config.vm.define "vinculum", primary: true do |vinculum|
    vinculum.vm.hostname = "vinculum"
    vinculum.vm.box = "generic/debian11"
    vinculum.vm.network "forwarded_port", guest: 80, host: 8081
    vinculum.vm.network :private_network, :ip => '10.11.11.10'
    vinculum.vm.provider "libvirt" do |vb|
        vb.memory = "2048"
        vb.cpus = "8"
        vb.memorybacking :access, :mode => "shared"
    end
    vinculum.vm.provision :ansible do |a|
      a.playbook = "vagrant/vinculum.yml"
    end
end

  config.vm.define "drone" do |drone|
    drone.vm.hostname = "drone"
    drone.vm.box = "generic/debian11"
    drone.vm.network :private_network, :ip => '10.11.11.11'
    drone.vm.provider "libvirt" do |vb|
        vb.memory = "2048"
        vb.cpus = "8"
        vb.memorybacking :access, :mode => "shared"
    end
    drone.vm.provision :ansible do |a|
      a.playbook = "vagrant/drone.yml"
    end
  end

  config.vm.define "remote" do |remote|
    remote.vm.hostname = "remote"
    remote.vm.box = "generic/debian11"
    remote.vm.network :private_network, :ip => '10.11.11.99'
    remote.vm.provider "libvirt" do |vb|
        vb.memory = "512"
        vb.cpus = "2"
        vb.memorybacking :access, :mode => "shared"
    end
    remote.vm.provision :ansible do |a|
      a.playbook = "vagrant/remote.yml"
    end
  end
end